use flatpath_core::Element;
use flatpath_derive::{Container, Element};

//#[derive(Element, Container)]
#[derive(Element, Container, Default)]
pub struct Svg {
    xmlns: String,
    //children: Vec<Box<dyn std::fmt::Display>>,
    children: Vec<Box<dyn flatpath_core::Element>>,
    // view_box: ()
}

//impl Default for Svg {
//}
// impl Svg {
//     pub fn new() -> Self {
//         let mut svg = Self::default();
//         svg.xmlns = "http://www.w3.org/2000/svg";
//         svg
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::path::Path;

    #[test]
    fn svg() {
        let mut s = Svg::new();
        let p = Path::new();
        s = s.append(p);

        assert_eq!(
            s.to_string(),
            r#"<svg xmlns="http://www.w3.org/2000/svg"><path /></svg>"#
        );
    }
}
