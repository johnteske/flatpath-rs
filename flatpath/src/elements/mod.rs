// Shape elements
// <circle>, <ellipse>, <line>, <mesh>, <path>, <polygon>, <polyline>, <rect>

use super::units::Number;

mod circle;

pub trait BoundingBox {
    fn x(&self) -> Number;
    fn y(&self) -> Number;
    fn width(&self) -> Number;
    fn height(&self) -> Number;
}
