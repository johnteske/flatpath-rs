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

        impl_element!($struct_name, $tag_name);

        impl std::fmt::Display for Path {
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
