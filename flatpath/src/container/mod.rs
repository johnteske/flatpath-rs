pub mod group;
pub mod svg;

#[macro_export]
macro_rules! impl_container(
    ($struct_name:ident, $tag_name:expr) => (
        #[derive(Default)]
        pub struct $struct_name {
            attributes: HashMap<String, String>,
            children: Vec<Box<dyn Element>>,
        }

        // Container
        impl $struct_name {
            pub fn append<T>(mut self, element: T) -> Self
            where
                T: 'static + Element,
            {
                self.children.push(Box::new(element));
                self
            }
        }

        impl_element!($struct_name, $tag_name);

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "<{} ", $tag_name)?;
                for (attr, value) in &self.attributes {
                    write!(f, r#"{}="{}" "#, attr, value)?;
                }
                write!(f, "{}", ">")?;
                for child in &self.children {
                    write!(f, "{}", child)?;
                }
                write!(f, "</{}>", $tag_name)
            }
        }
    );
);
