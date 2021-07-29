pub trait Element: std::fmt::Display {}

#[macro_export]
macro_rules! impl_element(
    ($struct_name:ident, $tag_name:expr) => (
        #[derive(Default)]
        pub struct $struct_name {
            attributes: HashMap<String, String>,
        }

        impl Element for $struct_name {}

        impl $struct_name {
            pub fn new() -> Self {
                $struct_name::default()
            }
            fn attr(&mut self, k: String, v: String) {
                self.attributes.insert(k, v);
            }
        }

        // Shape
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
