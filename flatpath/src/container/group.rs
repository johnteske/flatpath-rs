use crate::element::Element;
use element_derive::Element;

#[derive(Element, Default)]
pub struct Group {
    #[doc = "skip"]
    children: Vec<Box<dyn Element>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::circle::Circle;

    #[test]
    fn group() {
        let mut g = Group::new();
        let c = Circle::new();
        g = g.append(c);

        assert_eq!(g.to_string(), r#"<g><circle /></g>"#);
    }
}
