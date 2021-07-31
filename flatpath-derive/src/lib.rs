// TODO
// DRY it up
// look into serde Serializer over Display
// camelCase attributes for viewBox
// attribute to set tag_name
// attributes to ignore field for setter
// attributes to ignore field for serializer (use serde?)

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use syn::{Data, DataStruct, Fields};

#[proc_macro_derive(Element)]
pub fn element_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let setters = fields.into_iter().filter_map(|f| {
        let field_ident = &f.ident;
        let field_name = &f.ident.clone().unwrap();
        let field_ty = &f.ty;

        if field_name == "children" || field_name == "xmlns" {
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

#[proc_macro_derive(Container)]
pub fn container_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    //let mut tag_name: Option<String> = None;
    //    for option in input.attrs.into_iter() {
    //        let option = option.parse_meta().unwrap();
    //        match option {
    //            Meta::NameValue(MetaNameValue {
    //                ref ident, ref lit, ..
    //            }) if ident == "tag_name" => {
    //                if let Lit::Str(lit) = lit {
    //                    tag_name = Some(lit.value());
    //                }
    //            }
    //        }
    //    }

    let tag_name = format!("{}", struct_name.to_string().to_lowercase());

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let attribute_formatters = fields.into_iter().filter_map(|f| {
        let field_ident = &f.ident;
        let field_name = &f.ident.clone().unwrap();

        if field_name == "children" {
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

    let struct_name = input.ident;

    let tag_name = format!("{}", struct_name.to_string().to_lowercase());

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let attribute_formatters = fields.into_iter().map(|f| {
        let field_name = f.ident;
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