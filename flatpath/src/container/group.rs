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
    use crate::shape::path::Path;

    #[test]
    fn group() {
        let mut g = Group::new();
        let p = Path::new();
        g = g.append(p);

        assert_eq!(g.to_string(), r#"<g />"#);
    }
}
