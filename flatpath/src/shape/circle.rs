use svg::node::element::Circle as _Circle;

use crate::bounding_box::BoundingBox;
use crate::unit::Number;

#[derive(Default)]
pub struct Circle {
    r: Number,
    cx: Number,
    cy: Number,
}

impl Circle {
    fn new() -> Self {
        Circle::default()
    }
    fn set_radius(mut self, r: Number) -> Self {
        self.r = r;
        self
    }
}

impl BoundingBox for Circle {
    fn x(&self) -> Number {
        self.cx - self.r
    }
    fn y(&self) -> Number {
        self.cy - self.r
    }
    fn width(&self) -> Number {
        self.r * 2.0
    }
    fn height(&self) -> Number {
        self.width()
    }
}

// impl From<_Circle> for Circle {
//     fn from(item: _Circle) -> Self {
//         let attrs = &item.get_inner().get_attributes();
//         let r: Number = attrs["r"].parse().unwrap();
//         let cx: Number = attrs["cx"].parse().unwrap();
//         let cy: Number = attrs["cy"].parse().unwrap();
//         Circle { r, cx, cy }
//     }
// }

impl Into<_Circle> for Circle {
    fn into(self) -> _Circle {
        _Circle::new()
            .set("r", self.r)
            .set("cx", self.cx)
            .set("cy", self.cy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle() {
        let c = Circle::new().set_radius(3.);

        assert_eq!(c.width(), 6.0);
        assert_eq!(c.height(), 6.0);
        assert_eq!(c.x(), -3.0);

        let c: _Circle = c.into();
        assert_eq!(c.to_string(), r#"<circle cx="0" cy="0" r="3"/>"#);
    }
}
