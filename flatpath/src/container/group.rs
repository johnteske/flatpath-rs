use flatpath_core::Child;
use flatpath_derive::{Container, Element};

#[derive(Element, Container, Default)]
pub struct Group {
    children: Vec<Box<dyn Child>>,
}

impl Child for Group {}

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
