use crate::element::Element;
use element_derive::{Container, Element};

#[derive(Element, Container, Default)]
pub struct Svg {
    // #[doc = "skip"]
    xmlns: String,
    #[doc = "skip"]
    children: Vec<Box<dyn Element>>,
    // view_box: ()
}

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
