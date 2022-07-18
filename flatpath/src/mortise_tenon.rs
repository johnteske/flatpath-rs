use crate::shape::path::PathBuilder;
use crate::unit::{Number, PositiveNormalNumber};

pub struct MortiseAndTenon {
    mortise: PathBuilder,
    tenon: PathBuilder,
}

impl MortiseAndTenon {
    pub fn new(width: Number, height: Number, tenon_corner_radius: PositiveNormalNumber) -> Self {
        // Rectangle, origin is top left
        let mortise = PathBuilder::new()
            .move_to((0., 0.))
            .line_to((width, 0.))
            .line_to((width, height))
            .line_to((0., height))
            .close();

        // Facing up, assuming tenon extends above
        let tenon = PathBuilder::new()
            .move_to((0., 0.))
            .line_to_r((0., -height), tenon_corner_radius)
            .line_to_r((width, -height), tenon_corner_radius)
            .line_to((width, 0.));

        Self { mortise, tenon }
    }

    pub fn mortise(&self) -> &PathBuilder {
        &self.mortise
    }

    pub fn tenon(&self) -> &PathBuilder {
        &self.tenon
    }
}
