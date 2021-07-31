use crate::unit::Number;
use flatpath_derive::{Element, Shape};

#[derive(Element, Shape, Default)]
pub struct Circle {
    r: Number,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle() {
        let c = Circle::new().r(1.);
        assert_eq!(c.to_string(), r#"<circle r="1" />"#);
    }
}
