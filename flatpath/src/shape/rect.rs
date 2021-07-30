use std::collections::HashMap;

use crate::element::Element;
use crate::impl_element;
use crate::impl_shape;
use crate::unit::Number;

impl_shape!(Rect, "rect");

impl Rect {
    pub fn w(mut self, s: Number) -> Self {
        self.attr("w".to_string(), s.to_string());
        self
    }
    pub fn h(mut self, s: Number) -> Self {
        self.attr("h".to_string(), s.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect() {
        let r = Rect::new().w(1.);
        assert_eq!(r.to_string(), r#"<rect w="1" />"#);
    }
}
