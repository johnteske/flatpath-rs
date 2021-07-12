use svg::node::element::{Group, Path, Rectangle};
use svg::Document;

use crate::builder::{PathBuilder, Point};
use crate::units::Number;
use crate::units::{inches, mm};

pub fn project() -> Document {
    let num_rungs = 6;
    let T = mm(3.);
    let corner_radius = mm(1.0); // soften the outer edge a little
    let project_width = inches(10.); // TODO
    let project_height = inches(10.); // TODO (num_rungs * T) + ((num_rungs + 1) * rung_spacing)

    let mut g = Group::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1);

    let ladder_side_width = T * 4.0;
    let ladder_side_height = inches(10.);
    let mut ladder_side = Group::new().add(
        Rectangle::new()
            .set("width", ladder_side_width)
            .set("height", ladder_side_height)
            .set("rx", corner_radius)
            .set("ry", corner_radius),
    );

    let rung_spacing = inches(1.);
    for i in 0..num_rungs {
        ladder_side = ladder_side.add(
            Rectangle::new()
                .set("width", T)
                .set("height", T)
                .set("x", T * 1.5)
                .set("y", (rung_spacing * (i as Number + 1.)) + (T * i as Number)),
        );
    }

    g = g.add(ladder_side);

    let rung_width = inches(2.);
    let ladder_rung_data = PathBuilder::new()
        // top edge
        .add(Point(T, 0.))
        .add(Point(T + rung_width, 0.))
        .add(Point(T + rung_width, T * 0.5))
        // right tab
        .add_r(Point(T + rung_width + T, T * 0.5), corner_radius)
        .add_r(Point(T + rung_width + T, T * 1.5), corner_radius)
        .add(Point(T + rung_width, T * 1.5))
        // bottom edge
        .add(Point(T + rung_width, T * 2.0))
        .add(Point(T, T * 2.0))
        .add(Point(T, T * 1.5))
        // left tab
        .add_r(Point(0., T * 1.5), corner_radius)
        .add_r(Point(0., T * 0.5), corner_radius)
        .add(Point(T, T * 0.5))
        .close();

    g = g.add(Path::new().set("d", ladder_rung_data));

    Document::new()
        .set("viewBox", (0, 0, project_width, project_height))
        .add(g)
}
