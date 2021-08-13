mod d_builder;
pub use d_builder::*;

use flatpath_core::Child;
use flatpath_derive::{Element, Shape};

#[derive(Element, Shape, Default)]
pub struct Path {
    d: String,
}

impl Child for Path {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path() {
        let p = Path::new().d("M0,0 L10,10".into());
        assert_eq!(p.to_string(), r#"<path d="M0,0 L10,10"/>"#);
    }
}
