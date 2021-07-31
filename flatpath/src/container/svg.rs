use flatpath_core::Child;
use flatpath_derive::{Container, Element};

#[derive(Element, Container)]
pub struct Svg {
    xmlns: String,
    children: Vec<Box<dyn Child>>,
    // view_box: ()
}

impl Default for Svg {
    fn default() -> Self {
        Svg {
            xmlns: "http://www.w3.org/2000/svg".into(),
            children: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::path::Path;

    #[test]
    fn svg() {
        let mut s = Svg::new();
        let p = Path::new();
        s = s.append(p);

        assert_eq!(
            s.to_string(),
            r#"<svg xmlns="http://www.w3.org/2000/svg"><path /></svg>"#
        );
    }
}
