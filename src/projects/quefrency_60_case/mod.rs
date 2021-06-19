use svg::node::element::{Circle, Group, Path};
use svg::Document;

use crate::builder::{PathBuilder, Point};

static DPI: f32 = 96.;

// RIGHT SIDE
// width, top 160.4
// width, bot 155.638 (.6375?)
//
// MEASURED FROM RIGHT EDGE
// top right // 4.6, 4.6
// top edge // 80.5, 4.0
// top left // 156.4, 4.0
// right edge // 4.0, 55.625
// bottom right // 4.6, 106.65
// bottom edge // 78.12, 107.25
// bottom left // 151.64, 107.25

pub fn project() -> Document {
    let height = _mm(111.25);
    let thickness = _mm(8.);
    let outer_corner_radius = _mm(4.);
    let inner_corner_radius = _mm(2.);

    // LEFT SIDE
    let left_width_top = _mm(141.35);
    let left_width_bot = _mm(141.112);

    let mut left = Group::new();

    let left_data = PathBuilder::new()
        .add_r(Point(0., 0.), outer_corner_radius)
        .add_r(Point(left_width_top, 0.), inner_corner_radius)
        .add_r(Point(left_width_top, thickness), inner_corner_radius)
        .add(Point(thickness, thickness))
        .add(Point(thickness, height - thickness))
        .add_r(
            Point(left_width_bot, height - thickness),
            inner_corner_radius,
        )
        .add_r(Point(left_width_bot, height), inner_corner_radius)
        .add_r(Point(0., height), outer_corner_radius)
        .close();

    left = left.add(
        Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0)
            .set("d", left_data),
    );

    let holes = vec![
        Point(4.6, 4.6),       // top left
        Point(70.65, 4.0),     // top edge
        Point(137.35, 4.0),    // top right
        Point(4.0, 55.625),    // left edge
        Point(4.6, 106.65),    // left bottom
        Point(73.05, 107.250), // bottom edge
        Point(142.11, 107.25), // bottom right
    ];

    for p in holes {
        left = left.add(
            Circle::new()
                .set("r", _mm(1.1))
                .set("cx", _mm(p.0))
                .set("cy", _mm(p.1))
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0),
        );
    }

    Document::new()
        .set("viewBox", (0, 0, left_width_top, height))
        .add(left)
}

fn _mm(a: f32) -> f32 {
    (a * DPI) / 25.4
}
