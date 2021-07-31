use crate::unit::Number;
use flatpath_derive::{Element, Shape};

#[derive(Element, Shape, Default)]
pub struct Rect {
    width: Number,
    height: Number,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect() {
        let r = Rect::new().width(1.);
        assert_eq!(r.to_string(), r#"<rect w="1" />"#);
    }
}
