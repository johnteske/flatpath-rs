use flatpath_derive::{Element, Shape};

#[derive(Element, Shape, Default)]
pub struct Path {
    d: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path() {
        let p = Path::new().d("M0,0 L10,10");
        assert_eq!(p.to_string(), r#"<path d="M0,0 L10,10" />"#);
    }
}
