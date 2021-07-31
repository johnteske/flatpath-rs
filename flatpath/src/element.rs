pub trait Element: std::fmt::Display {}

// #[macro_export]
// macro_rules! impl_element(
//     ($struct_name:ident, $tag_name:expr) => (
//         impl $struct_name {
//             fn attr<T>(&mut self, name: T, value: T)
//             where T: Into<String> {
//                 self.attributes.insert(name.into(), value.into());
//             }
//         }
//
//         impl Element for $struct_name {}
//     );
// );
