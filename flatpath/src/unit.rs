//! Absolute length units
//!
//! Follows the CSS3 specifications for
//! [resolution](https://drafts.csswg.org/css-values-3/#resolution) (96dpi) and
//! [absolute lengths](https://drafts.csswg.org/css-values-3/#absolute-lengths).

/// Standard length unit type
pub type Number = f32;

static DPI: Number = 96.;

/// Converts a length, as inches, into "dots"
pub fn inches(a: Number) -> Number {
    a * DPI
}

/// Converts a length, as millimeters, into "dots"
pub fn mm(a: Number) -> Number {
    (a * DPI) / 25.4
}
