//Shape elements
//<circle>, <ellipse>, <line>, <mesh>, <path>, <polygon>, <polyline>, <rect>

use super::units::Number;
use svg::node::element::{Circle as _Circle, Group as _Group};

trait Dimensions {
    fn width(&self) -> Number {
        unimplemented!()
    }
    fn height(&self) -> Number {
        unimplemented!()
    }
}

struct Circle(_Circle);

impl Dimensions for Circle {
    fn width(&self) -> Number {
        let attrs = &self.0.get_inner().get_attributes();
        let r: Number = attrs["r"].parse().unwrap();
        r * 2.0
    }

    fn height(&self) -> Number {
        self.width()
    }
}

impl From<_Circle> for Circle {
    fn from(item: _Circle) -> Self {
        Circle(item)
    }
}

#[derive(Default)]
struct Circle2 {
    r: Number,
    cx: Number,
    cy: Number,
}

impl Circle2 {
    fn new() -> Self {
        Circle2::default()
    }
    fn radius(mut self, r: Number) -> Self {
        self.r = r;
        self
    }
    //
    fn x(&self) -> Number {
        &self.cx - &self.r
    }
}

impl Dimensions for Circle2 {
    fn width(&self) -> Number {
        &self.r * 2.0
    }

    fn height(&self) -> Number {
        self.width()
    }
}

impl From<_Circle> for Circle2 {
    fn from(item: _Circle) -> Self {
        let attrs = &item.get_inner().get_attributes();
        let r: Number = attrs["r"].parse().unwrap();
        let cx: Number = attrs["r"].parse().unwrap();
        let cy: Number = attrs["r"].parse().unwrap();
        Circle2 { r, cx, cy }
    }
}

struct Group(_Group);

impl Group {}

impl Dimensions for Group {
    fn width(&self) -> Number {
        let attrs = &self.0.get_inner().get_attributes();
        let r: Number = attrs["r"].parse().unwrap();
        r * 2.0
    }

    fn height(&self) -> Number {
        self.width()
    }
}

impl From<_Group> for Group {
    fn from(item: _Group) -> Self {
        Group(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_dimensions() {
        let source = _Circle::new().set("r", 3);
        let with_dimensions = Circle::from(source);

        assert_eq!(with_dimensions.width(), 6.0);
        assert_eq!(with_dimensions.height(), 6.0);
    }

    #[test]
    fn circle2() {
        let c2 = Circle2::new().radius(3.);

        assert_eq!(c2.width(), 6.0);
        assert_eq!(c2.height(), 6.0);
        assert_eq!(c2.x(), -3.0);
    }
}
