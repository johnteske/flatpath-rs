use crate::element::Element;
use std::collections::HashMap;

#[derive(Default)]
struct Svg {
    attributes: HashMap<String, String>,
    children: Vec<Box<dyn Element>>,
}

impl Svg {
    pub fn new() -> Self {
        Svg::default()
    }
    // Container
    pub fn append<T>(mut self, element: T) -> Self
    where
        T: 'static + Element,
    {
        self.children.push(Box::new(element));
        self
    }
    //    // TODO accept "Rect" geometry, not tuple?
    //    pub fn view_box(self, rect: (Number, Number, Number, Number)) -> Self {
    //        self.inner..set("viewBox", rect)
    //        self
    //    }
}

// Container
impl std::fmt::Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ", "svg")?; // $tag_name)?;
        for (attr, value) in &self.attributes {
            write!(f, r#"{}="{}" "#, attr, value)?;
        }
        write!(f, "{}", ">")?;
        //
        for child in &self.children {
            write!(f, "{}", child)?;
        }
        //
        write!(f, "{}", "</svg>")
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

        assert_eq!(s.to_string(), r#"<svg />"#);
    }
}
