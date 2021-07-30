//! SVG shape element builders

// Shape elements
// <circle>, <ellipse>, <line>, <mesh>, <path>, <polygon>, <polyline>, <rect>

pub mod circle;
pub mod path;

#[macro_export]
macro_rules! impl_shape(
    ($struct_name:ident, $tag_name:expr) => (
        #[derive(Default)]
        pub struct $struct_name {
            attributes: HashMap<String, String>,
        }

        impl $struct_name {
            pub fn new() -> Self {
                $struct_name::default()
            }
        }

        impl_element!($struct_name, $tag_name);

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "<{} ", $tag_name)?;
                for (attr, value) in &self.attributes {
                    write!(f, r#"{}="{}" "#, attr, value)?;
                }
                write!(f, "{}", "/>")
            }
        }
    );
);
