use svg::node::element::{Circle, Group, Path};
use svg::Document;

use flatpath::shape::path::PathBuilder;
use flatpath::unit::mm;

mod av_module;

type Point = (f32, f32);

#[derive(Default)]
struct SideBuilder {
    path: String,
    holes: Vec<Point>,
}
impl SideBuilder {
    fn new() -> Self {
        SideBuilder::default()
    }

    fn to_group(self) -> Group {
        let mut g = Group::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1);

        g = g.add(Path::new().set("d", self.path));

        for point in self.holes {
            g = g.add(
                Circle::new()
                    .set("r", mm(1.1))
                    .set("cx", point.0)
                    .set("cy", point.1),
            );
        }

        g
    }
}

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

    let mut left = SideBuilder::new();

    left.path = PathBuilder::new()
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

    left.holes = vec![
        (4.6, 4.6),       // top left
        (70.65, 4.0),     // top edge
        (137.35, 4.0),    // top right
        (4.0, 55.625),    // left edge
        (4.6, 106.65),    // left bottom
        (73.05, 107.250), // bottom edge
        (142.11, 107.25), // bottom right
    ]
    .iter()
    .map(|p| (mm(p.0), mm(p.1)))
    .collect();

    // module
    let left_g = left.to_group().add(
        avm.render()
            .set("transform", format!("translate({},{})", alm_x, -avm.height)),
    );

    Document::new()
        .set("viewBox", (0, -70, left_width_bot, height + 70.))
        .add(left_g)
}
