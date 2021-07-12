use svg::node::element::{Group, Path, Rectangle};
use svg::Document;

use crate::builder::{PathBuilder, Point};
use crate::units::Number;
use crate::units::{inches, mm};

pub fn project() -> Document {
    let rung_width = inches(2.);
    let rung_spacing = inches(1.5);
    let num_rungs = 6;
    let t = mm(3.);
    let corner_radius = mm(1.0); // soften the outer edge a little

    let mut g = Group::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1);

    let side_width = t * 4.0;
    let side_height = (num_rungs as Number * t) + ((num_rungs as Number + 1.) * rung_spacing);
    let mut side = Group::new().add(
        Rectangle::new()
            .set("width", side_width)
            .set("height", side_height)
            .set("rx", corner_radius)
            .set("ry", corner_radius),
    );

    for i in 0..num_rungs {
        side = side.add(
            Rectangle::new()
                .set("width", t)
                .set("height", t)
                .set("x", t * 1.5)
                .set("y", (rung_spacing * (i as Number + 1.)) + (t * i as Number)),
        );
    }

    g = g.add(side);

    let rung_data = PathBuilder::new()
        // top edge
        .add(Point(t, 0.))
        .add(Point(t + rung_width, 0.))
        .add(Point(t + rung_width, t * 0.5))
        // right tab
        .add_r(Point(t + rung_width + t, t * 0.5), corner_radius)
        .add_r(Point(t + rung_width + t, t * 1.5), corner_radius)
        .add(Point(t + rung_width, t * 1.5))
        // bottom edge
        .add(Point(t + rung_width, t * 2.0))
        .add(Point(t, t * 2.0))
        .add(Point(t, t * 1.5))
        // left tab
        .add_r(Point(0., t * 1.5), corner_radius)
        .add_r(Point(0., t * 0.5), corner_radius)
        .add(Point(t, t * 0.5))
        .close();

    g = g.add(Path::new().set("d", rung_data).set(
        "transform",
        format!("translate({}, {})", side_width + t, 0.),
    ));

    Document::new()
        .set("viewBox", (0, 0, t + rung_width + t, side_height))
        .add(g)
}
