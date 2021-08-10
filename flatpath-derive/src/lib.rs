// TODO
// DRY it up
// look into serde Serializer over Display
// camelCase attributes for viewBox

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use syn::{Data, Fields, FieldsNamed};

#[proc_macro_derive(Element, attributes(no_setter))]
pub fn element_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let fields = named_fields(&input);

    let setters = fields.named.iter().filter_map(|f| {
        let field_ident = &f.ident;
        let field_ty = &f.ty;

        if f.attrs.iter().any(|a| a.path.is_ident("no_setter")) {
            None
        } else {
            Some(quote! {
                pub fn #field_ident(mut self, value: #field_ty) -> Self {
                    self.#field_ident = value;
                    self
                }
            })
        }
    });

    (quote! {
        impl #struct_name {
            pub fn new() -> Self {
                #struct_name::default()
            }
            #(#setters)*
        }
    })
    .into()
}

#[proc_macro_derive(Container, attributes(tag_name, no_write))]
pub fn container_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let mut tag_name = struct_name.to_string().to_lowercase();
    for attr in &input.attrs {
        if attr.path.is_ident("tag_name") {
            let lit: syn::LitStr = attr.parse_args().unwrap();
            tag_name = lit.value();
        }
    }

    let fields = named_fields(&input);

    let attribute_formatters = fields.named.iter().filter_map(|f| {
        let field_ident = &f.ident;

        if f.attrs.iter().any(|a| a.path.is_ident("no_write")) {
            None
        } else {
            Some(quote! {
                write!(f, r#" {}="{}""#, stringify!(#field_ident), &self.#field_ident)?;
            })
        }
    });

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
                #(#attribute_formatters)*
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

#[proc_macro_derive(Shape)]
pub fn shape_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let tag_name = struct_name.to_string().to_lowercase();

    let fields = named_fields(&input);

    let attribute_formatters = fields.named.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            write!(f, r#" {}="{}""#, stringify!(#field_name), &self.#field_name)?;
        }
    });

    (quote! {
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // open tag
                write!(f, "<{}", #tag_name)?;

                // attributes
                #(#attribute_formatters)*

                // close tag
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
