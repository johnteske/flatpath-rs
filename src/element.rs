use super::units::Number;
use svg::node::element::Circle as _Circle;

trait Dimensions {
    fn width(&self) -> Number {
        unimplemented!()
    }
    fn height(&self) -> Number {
        unimplemented!()
    }
}

struct Circle(_Circle);

impl Circle {
    fn new(source: _Circle) -> Circle {
        Circle(source)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_dimensions() {
        let source = _Circle::new().set("r", 3);
        let with_dimensions = Circle::new(source);

        assert_eq!(with_dimensions.width(), 6.0);
        assert_eq!(with_dimensions.height(), 6.0);
    }
}
