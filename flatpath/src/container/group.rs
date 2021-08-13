use flatpath_core::Child;
use flatpath_derive::{Container, Element};

#[derive(Element, Container, Default)]
#[tag_name("g")]
pub struct Group {
    #[internal]
    children: Vec<Box<dyn Child>>,
}

impl Child for Group {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::Circle;

    #[test]
    fn group() {
        let mut g = Group::new();
        let c = Circle::new();
        g = g.append(c);

        assert_eq!(g.to_string(), r#"<g><circle r="0"/></g>"#);
    }
}
