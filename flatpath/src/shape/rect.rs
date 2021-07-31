use crate::unit::Number;
use flatpath_core::Child;
use flatpath_derive::{Element, Shape};

#[derive(Element, Shape, Default)]
pub struct Rect {
    width: Number,
    height: Number,
}

impl Child for Rect {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect() {
        let r = Rect::new().width(1.);
        assert_eq!(r.to_string(), r#"<rect w="1" />"#);
    }
}
