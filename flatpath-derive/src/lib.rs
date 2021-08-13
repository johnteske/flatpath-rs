// TODO
// DRY it up
// look into serde Serializer over Display
// camelCase attributes for viewBox

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use syn::{Data, Field, Fields, FieldsNamed};

#[proc_macro_derive(Element, attributes(no_setter))]
pub fn element_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let fields = named_fields(&input);

    let attr_setters = fields.named.iter().filter_map(|f| {
        let Field { ident, ty, .. } = &f;

        if field_has_attr(f, "no_setter") {
            return None;
        }

        Some(quote! {
            pub fn #ident(mut self, value: #ty) -> Self {
                self.#ident = value;
                self
            }
        })
    });

    (quote! {
        impl #struct_name {
            pub fn new() -> Self {
                #struct_name::default()
            }
            #(#attr_setters)*
        }
    })
    .into()
}

#[proc_macro_derive(Container, attributes(tag_name, no_write))]
pub fn container_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (struct_name, tag_name) = get_names(&input);
    let fields = named_fields(&input);

    let attr_formatters = attribute_formatters(fields).into_iter();

    (quote! {
        impl #struct_name {
            pub fn append<T>(mut self, element: T) -> Self
            where
                T: 'static + flatpath_core::Child // TODO lifetime
            {
                self.children.push(Box::new(element));
                self
            }
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // opening tag with attributes
                write!(f, "<{}", #tag_name)?;
                #(#attr_formatters)*
                write!(f, "{}", ">")?;

                // children
                for child in &self.children {
                    write!(f, "{}", child)?;
                }

                // closing tag
                write!(f, "</{}>", #tag_name)
            }
        }
    })
    .into()
}

#[proc_macro_derive(Shape, attributes(tag_name, no_write))]
pub fn shape_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (struct_name, tag_name) = get_names(&input);
    let fields = named_fields(&input);

    let attr_formatters = attribute_formatters(fields).into_iter();

    (quote! {
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // open tag
                write!(f, "<{}", #tag_name)?;

                // attributes
                #(#attr_formatters)*

                // self-closing tag
                write!(f, "{}", "/>")
            }
        }
    })
    .into()
}

fn named_fields(input: &DeriveInput) -> &FieldsNamed {
    const UNSUPPORTED: &str = "must use a struct with named fields";
    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields,
            _ => unimplemented!("{}", UNSUPPORTED),
        },
        _ => unimplemented!("{}", UNSUPPORTED),
    }
}

fn field_has_attr(f: &Field, name: &str) -> bool {
    f.attrs.iter().any(|a| a.path.is_ident(name))
}

// Returns a Vec that is later turned into an Iter for now since the type signature is complicated
fn attribute_formatters(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields
        .named
        .iter()
        .filter_map(|f| {
            let field_ident = &f.ident;

            if field_has_attr(f, "no_write") {
                return None;
            }

            Some(quote! {
                write!(f, r#" {}="{}""#, stringify!(#field_ident), &self.#field_ident)?;
            })
        })
        .collect::<Vec<proc_macro2::TokenStream>>()
}

fn get_names(input: &DeriveInput) -> (&proc_macro2::Ident, String) {
    let struct_name = &input.ident;

    let mut tag_name = struct_name.to_string().to_lowercase();
    for attr in &input.attrs {
        if attr.path.is_ident("tag_name") {
            let lit: syn::LitStr = attr.parse_args().unwrap();
            tag_name = lit.value();
        }
    }
    (struct_name, tag_name)
}
