use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{parse_macro_input, DeriveInput};
use syn::{Data, DataStruct, Fields};
//use syn::{Meta, Path};

//#[proc_macro_derive(Element, attributes(attr))]
#[proc_macro_derive(Element)]
pub fn element_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    //Meta(Path(word)) if word == SKIP_DESERIALIZING
    // let inner_attrs = Attributes::parse_inner(input).unwrap();

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let setters = fields.into_iter().map(|f| {
        let field_name = f.ident;
        let field_ty = f.ty;

        // if ANY attribute exists, skip
        if f.attrs.is_empty() {
            quote! {
                pub fn #field_name(mut self, value: #field_ty) -> Self {
                    self.#field_name = value;
                    self
                }
            }
        } else {
            quote! {}
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

    let tag_name = "TODO";

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let attribute_formatters = fields.into_iter().map(|f2| {
        let field_name = f2.ident;

        // if ANY attribute exists, skip
        if f2.attrs.is_empty() {
            quote! {
                write!(f, r#" {}="{}""#, stringify!(#field_name), &self.#field_name)?;
            }
        } else {
            quote! {}
        }
    });

    (quote! {
        impl #struct_name {
            pub fn append<T>(mut self, element: T) -> Self
            where
                T: 'static + Element,
            {
                self.children.push(Box::new(element));
                self
            }
        }
        // TODO serde serialize
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "<{}", #tag_name)?;

                #(#attribute_formatters)*
                //( #(#setters),* ) => {}

                write!(f, "{}", ">")?;
                for child in &self.children {
                    write!(f, "{}", child)?;
                }
                write!(f, "</{}>", #tag_name)
            }
        }
    })
    .into()
}

// fn get_name_field() {
//     let fields = match input.data {
//         Data::Struct(DataStruct {
//             fields: Fields::Named(fields),
//             ..
//         }) => fields.named,
//         _ => panic!("this derive macro only works on structs with named fields"),
//     };
//     fields.filter attrs
// }
