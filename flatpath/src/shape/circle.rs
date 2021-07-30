use std::collections::HashMap;

use crate::element::Element;
use crate::impl_element;
use crate::impl_shape;
use crate::unit::Number;

impl_shape!(Circle, "circle");

impl Circle {
    pub fn r(mut self, s: Number) -> Self {
        self.attr("r".to_string(), s.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write() {
        let c = Circle::new().r(1.);
        assert_eq!(c.to_string(), "circle");
    }
}
