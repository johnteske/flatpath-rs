use svg::node::element::{Circle, Group, Path};
use svg::Document;

use flatpath::shape::path::PathBuilder;
use flatpath::unit::mm;

mod av_module;

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
    let height = mm(111.25);
    let thickness = mm(8.);
    let outer_corner_radius = mm(4.);
    let inner_corner_radius = mm(2.);

    // LEFT SIDE
    let left_width_top = mm(141.35);
    let left_width_bot = mm(146.112);

    // audio/LED module
    let avm = av_module::AVModule::new(inner_corner_radius);
    let alm_lead = thickness; // TODO rename, this is the lead-in/out/curve to the main body
    let alm_x = thickness * 2.;

    let mut left = Group::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0);

    let left_data = PathBuilder::new()
        .add_r((0., 0.), outer_corner_radius)
        // audio/LED module
        .add_r((alm_x - alm_lead, 0.), inner_corner_radius)
        .add_r((alm_x, -2. * avm.padding - avm.height), inner_corner_radius)
        .add_r(
            (alm_x + avm.width, -2. * avm.padding - avm.height),
            inner_corner_radius,
        )
        .add_r((alm_x + avm.width + alm_lead, 0.), inner_corner_radius)
        // end audio/LED module
        .add_r((left_width_top, 0.), inner_corner_radius)
        .add_r((left_width_top, thickness), inner_corner_radius)
        .add((thickness, thickness))
        .add((thickness, height - thickness))
        .add_r((left_width_bot, height - thickness), inner_corner_radius)
        .add_r((left_width_bot, height), inner_corner_radius)
        .add_r((0., height), outer_corner_radius)
        .close();

    left = left.add(Path::new().set("d", left_data));

    // in mm
    let holes = vec![
        (4.6, 4.6),       // top left
        (70.65, 4.0),     // top edge
        (137.35, 4.0),    // top right
        (4.0, 55.625),    // left edge
        (4.6, 106.65),    // left bottom
        (73.05, 107.250), // bottom edge
        (142.11, 107.25), // bottom right
    ];

    for p in holes {
        left = left.add(
            Circle::new()
                .set("r", mm(1.1))
                .set("cx", mm(p.0))
                .set("cy", mm(p.1)),
        );
    }

    // module
    let avm_height = avm.height;
    left = left.add(avm.render().set(
        "transform",
        format!(
            "translate({},{})",
            thickness * 2.,
            -inner_corner_radius - avm_height
        ),
    ));

    Document::new()
        .set("viewBox", (0, 0, left_width_bot, height))
        .add(left)
}

//fn with_mounting_holes(group: Group, points: Vec<Point>) -> Group {
//    let mut new_group;
//    for p in points {
//        new_group = group.add(
//            Circle::new()
//                .set("r", mm(1.1))
//                .set("cx", mm(p.0))
//                .set("cy", mm(p.1))
//                .set("fill", "none")
//                .set("stroke", "black")
//                .set("stroke-width", 0),
//        );
//    }
//    new_group
//}
