use std::collections::HashMap;

use crate::element::Element;
use crate::impl_container;
use crate::impl_element;

impl_container!(Group, "g");

impl Group {
    pub fn new() -> Self {
        Group::default()
    }
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
