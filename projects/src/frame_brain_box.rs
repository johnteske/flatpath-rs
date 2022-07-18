//! Box for Samsung's "The Frame" brain
//!
//! Design goals:
//! - sits on floor, cables to the side
//! - top is covered to prevent dust
//! - comes apart easily to allow access to box

use super::Project;
use svg::node::element::{Group, Rectangle};
use svg::Document;

use flatpath::mortise_tenon::MortiseAndTenon;
use flatpath::shape::path::{Command, PathBuilder, Point};
use flatpath::unit::{inches, mm, PositiveNormalNumber};

struct GroupPair {
    cut: Group,
    debug: Group,
}

pub struct BrainBox;
impl Project for BrainBox {
    fn generate(&self) -> Document {
        // parameters
        let t = mm(3.);
        let corner_radius = PositiveNormalNumber::new(mm(1.0)).unwrap(); // soften the outer edges a little
        let tenon_corner_radius = PositiveNormalNumber::new(mm(0.5)).unwrap();
        // inner dimensions
        let width = inches(3.75);
        let height = inches(9.);
        let depth = inches(1.5);

        let overhang = 2. * t;

        let mut g = Group::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "1");
        g = g.set("transform", "translate(100, 100)");

        let mut debug = Group::new()
            .set("fill", "none")
            .set("stroke", "cyan")
            .set("stroke-width", "1");
        debug = debug.set("transform", "translate(100, 100)");

        let fifth_width = width / 5.;
        let long_joint = MortiseAndTenon::new(fifth_width, t, tenon_corner_radius);
        let third_depth = depth / 3.;
        let short_joint = MortiseAndTenon::new(third_depth, t, tenon_corner_radius);

        let top_bottom = {
            let debug_g =
                Group::new().add(Rectangle::new().set("width", width).set("height", depth));
            let mut g = Group::new()
                .add(
                    Rectangle::new()
                        .set("width", width + (2. * overhang))
                        .set("height", depth + (2. * overhang))
                        .set("rx", corner_radius.get())
                        .set("ry", corner_radius.get()),
                )
                .set(
                    "transform",
                    format!("translate({}, {})", -overhang, -overhang),
                );

            let mortise = long_joint.clone().mortise().to_path();
            let (x0, x1) = (fifth_width, fifth_width * 3.);
            let (y0, y1) = (t, overhang + depth);
            for (x, y) in [(x0, y0), (x1, y0), (x0, y1), (x1, y1)] {
                g = g.add(
                    mortise
                        .clone()
                        .set("transform", format!("translate({},{})", overhang + x, y)),
                );
            }

            let mortise = rotate90(&short_joint.mortise()).to_path();
            for (x, y) in [(0., 0.), (width + t, 0.)] {
                g = g.add(mortise.clone().set(
                    "transform",
                    format!("translate({},{})", overhang + x, overhang + third_depth + y),
                ));
            }

            GroupPair {
                cut: g,
                debug: debug_g,
            }
        };

        let front_back = {
            let debug_g =
                Group::new().add(Rectangle::new().set("width", width).set("height", height));

            let w = width + (2. * overhang);
            let mut g = Group::new().set("transform", format!("translate({}, {})", -overhang, 0.));

            let bottom_tenon = long_joint
                .clone()
                .tenon()
                // flip Y
                .map(|point| (point.x(), -point.y()).into())
                // flip X
                .map(|point| (-point.x() + fifth_width, point.y()).into());

            g = g.add(
                PathBuilder::new()
                    .move_to((0., 0.))
                    .extend(
                        long_joint
                            .clone()
                            .tenon()
                            .map(|point| (overhang + fifth_width + point.x(), point.y()).into())
                            .into_iter(),
                    )
                    .extend(
                        long_joint
                            .clone()
                            .tenon()
                            .map(|point| {
                                (overhang + (fifth_width * 3.) + point.x(), point.y()).into()
                            })
                            .into_iter(),
                    )
                    .line_to((w, 0.))
                    .line_to((w, height))
                    .extend(
                        bottom_tenon
                            .map(|point| {
                                (
                                    point.x() + overhang + (fifth_width * 3.),
                                    point.y() + height,
                                )
                                    .into()
                            })
                            .into_iter(),
                    )
                    .extend(
                        bottom_tenon
                            .map(|point| {
                                (point.x() + overhang + fifth_width, point.y() + height).into()
                            })
                            .into_iter(),
                    )
                    .line_to((0., height))
                    .close()
                    .to_path(),
            );

            GroupPair {
                cut: g,
                debug: debug_g,
            }
        };

        for (i, pair) in [top_bottom, front_back].iter().enumerate() {
            let position = format!("translate({}, 0)", i as f32 * (width + 100.));

            let transform = match pair.cut.get_inner().get_attributes().get("transform") {
                Some(transform) => format!("{} {}", transform, position.clone()),
                None => position.clone(),
            };
            g = g.add(pair.cut.clone().set("transform", transform));

            let transform = match pair.debug.get_inner().get_attributes().get("transform") {
                Some(transform) => format!("{} {}", transform, position),
                None => position.clone(),
            };
            debug = debug.add(pair.debug.clone().set("transform", transform))
        }

        Document::new()
            .set("viewBox", ("0", "0", "999", "999"))
            .add(debug)
            .add(g)
    }
}

fn rotate90(builder: &PathBuilder) -> PathBuilder {
    builder.map(|point| (-point.y(), point.x()).into())
}
fn rotate180(builder: &PathBuilder) -> PathBuilder {
    builder.map(|point| (point.x(), point.y()).into())
}
