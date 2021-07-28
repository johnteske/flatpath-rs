use svg::node::element::{Group, Path, Rectangle};
use svg::Document;

use flatpath::shape::path::DBuilder;
use flatpath::unit::{inches, mm, Number};

pub fn project() -> Document {
    // parameters
    let t = mm(3.);
    let side_width = t * 4.0;
    let num_rungs = 6;

    // constants and derived values
    let rung_width = inches(2.);
    let rung_spacing = inches(1.5);
    let side_height = (num_rungs as Number * t) + ((num_rungs as Number + 1.) * rung_spacing);
    let rung_depth = side_width - (t * 2.0);
    let rung_tab_depth = rung_depth / 2.0;
    let corner_radius = mm(1.0); // soften the outer edges a little

    let mut g = Group::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1);

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
                .set("width", rung_tab_depth)
                .set("height", t)
                .set("x", (side_width - rung_tab_depth) / 2.0)
                .set("y", (rung_spacing * (i as Number + 1.)) + (t * i as Number)),
        );
    }

    g = g.add(side);

    let rung_data = DBuilder::new()
        // top edge
        .add((t, 0.))
        .add((t + rung_width, 0.))
        .add((t + rung_width, rung_depth * 0.25))
        // right tab
        .add_r((t + rung_width + t, rung_depth * 0.25), corner_radius)
        .add_r((t + rung_width + t, rung_depth * 0.75), corner_radius)
        .add((t + rung_width, rung_depth * 0.75))
        // bottom edge
        .add((t + rung_width, rung_depth))
        .add((t, rung_depth))
        .add((t, rung_depth * 0.75))
        // left tab
        .add_r((0., rung_depth * 0.75), corner_radius)
        .add_r((0., rung_depth * 0.25), corner_radius)
        .add((t, rung_depth * 0.25))
        .close();

    g = g.add(Path::new().set("d", rung_data).set(
        "transform",
        format!("translate({}, {})", side_width + t, 0.),
    ));

    Document::new()
        .set("viewBox", (0, 0, t + rung_width + t, side_height))
        .add(g)
}
