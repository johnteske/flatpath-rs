use flatpath_core::Child;
use flatpath_derive::{Container, Element};

#[derive(Element, Container)]
pub struct Svg {
    #[internal]
    children: Vec<Box<dyn Child>>,

    #[skip_setter]
    xmlns: String,

    #[rename("viewBox")]
    view_box: String, // TODO
}

impl Default for Svg {
    fn default() -> Self {
        Svg {
            xmlns: "http://www.w3.org/2000/svg".into(),
            children: vec![],
            view_box: "".into(),
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
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox=""><path d=""/></svg>"#
        );
    }
}
