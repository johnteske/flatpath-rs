use std::collections::HashMap;

use crate::element::Element;
use crate::impl_container;
use crate::impl_element;

impl_container!(Svg, "svg");

impl Svg {
    pub fn new() -> Self {
        let mut svg = Self::default();
        svg.attr("xmlns", "http://www.w3.org/2000/svg");
        svg
    }
}

// impl Svg {
//     pub fn view_box(mut self, s: TODO Rect or tuple of Number) -> Self {
//         self.attr("viewBox".to_string(), s.to_string());
//         self
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

        assert_eq!(s.to_string(), r#"<svg />"#);
    }
}
