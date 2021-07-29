use std::collections::HashMap;

use crate::element::Element;
use crate::impl_element;
use crate::impl_shape;

impl_shape!(Path, "path");

impl Path {
    pub fn d(mut self, s: &str) -> Self {
        self.attr("d".to_string(), s.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write() {
        let p = Path::new().d("M0,0 L10,10");
        assert_eq!(p.to_string(), "path");
    }
}
