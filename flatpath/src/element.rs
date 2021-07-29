//pub(crate) trait Element { // : std::fmt::Display {
//    fn new() -> Self;
//    // TODO is private?
//    fn attr(&mut self, k: String, v: String);
//}

#[macro_export]
macro_rules! impl_element(
    ($struct_name:ident, $tag_name:expr) => (
        #[derive(Default)]
        pub struct $struct_name {
            attributes: HashMap<String, String>,
        }

        impl $struct_name {
        // impl Element for $struct_name {
            pub fn new() -> Self {
                $struct_name::default()
            }
            fn attr(&mut self, k: String, v: String) {
                self.attributes.insert(k, v);
            }
        }

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
