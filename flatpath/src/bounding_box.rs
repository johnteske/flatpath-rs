use crate::unit::Number;

pub(crate) trait BoundingBox {
    fn x(&self) -> Number;
    fn y(&self) -> Number;
    fn width(&self) -> Number;
    fn height(&self) -> Number;
}
