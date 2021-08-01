//! SVG shape element builders

// Shape elements
// <circle>, <ellipse>, <line>, <mesh>, <path>, <polygon>, <polyline>, <rect>

mod circle;
pub use self::circle::Circle;

pub mod path;

mod rect;
pub use self::rect::Rect;
