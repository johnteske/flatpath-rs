pub trait Element: std::fmt::Display {}

#[macro_export]
macro_rules! impl_element(
    ($struct_name:ident, $tag_name:expr) => (
        impl $struct_name {
            pub fn new() -> Self {
                $struct_name::default()
            }
            fn attr(&mut self, k: String, v: String) {
                self.attributes.insert(k, v);
            }
        }

        impl Element for $struct_name {}
    );
);
