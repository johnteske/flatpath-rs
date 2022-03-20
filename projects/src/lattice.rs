use super::Project;
use svg::node::element::{Circle, Group, Line, Path};
use svg::Document;

use flatpath::shape::path::PathBuilder;
use flatpath::unit::{inches, mm, Number};

struct Dimensions {
    x: Number,
    y: Number,
}

pub struct Lattice;
impl Project for Lattice {
    fn generate(&self) -> Document {
        // parameters
        let t = mm(3.);
        let width = inches(3.);
        let radius = width / 2.;
        let height = inches(0.5);
        let notches = 6;

        let mut g = Group::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1);

        // debug
        let mut debug_g = Group::new().set("stroke", "cyan");
        let div = width / (notches + 1) as Number;
        for i in 0..=(notches + 1) {
            let x = div * i as Number;
            debug_g = debug_g.add(
                Line::new()
                    .set("x1", x)
                    .set("x2", x)
                    .set("y1", 0.)
                    .set("y2", inches(9.)),
            );
        }
        let mut debug_top = Group::new().set("transform", format!("translate({radius}, {radius})"));
        debug_top = debug_top.add(Circle::new().set("r", radius));

        // TODO this should be shared
        let div = width / (notches + 1) as Number;

        for i in 1..=notches {
            // remove half T:
            // if the line is the center of the material,
            // part of the edge would be outside the circle
            let y = (div * i as Number) - (t / 2.);
            let w = line_len(
                &Dimensions {
                    x: width,
                    y: y - radius,
                },
                radius,
            );

            debug_top = debug_top.add(
                Line::new()
                    .set("x1", -w)
                    .set("x2", w)
                    .set("y1", y - radius)
                    .set("y2", y - radius),
            );

            g = g.add(
                slat(
                    Dimensions {
                        x: w * 2.,
                        y: height,
                    },
                    t,
                    notches,
                    width,
                )
                .set(
                    "transform",
                    format!(
                        "translate({}, {})",
                        (width - (w * 2.)) / 2.,
                        i as Number * (height * 2.)
                    ),
                ),
            );
        }

        debug_g = debug_g.add(debug_top);
        g = g.add(debug_g);

        Document::new().set("viewBox", (0, 0, 999, 999)).add(g)
    }
}

fn slat(dim: Dimensions, t: Number, notches: usize, max_width: Number) -> Path {
    let Dimensions { x: width, y } = dim;
    let r = mm(1.0);
    let r2 = mm(0.5);

    let mut slat = PathBuilder::new();

    // top left
    slat = slat.add_r((0., 0.), r);

    // notches
    let div = max_width / (notches + 1) as Number;
    for i in 1..=notches {
        let x = (div * i as Number) - ((max_width - (width)) / 2.);

        let x1 = x - (t / 2.);
        let x2 = x + (t / 2.);

        let range = t..(width - t);
        if !range.contains(&x1) || !range.contains(&x2) {
            continue;
        }

        let d = y * 0.5;
        slat = slat
            .add_r((x1, 0.), r2)
            .add((x1, d))
            .add((x2, d))
            .add_r((x2, 0.), r2);
    }

    slat = slat
        // top right and bottom
        .add_r((width, 0.), r)
        .add_r((width, y), r)
        .add_r((0., y), r);

    Path::new().set("d", slat.close())
}

// y is height
fn line_len(dim: &Dimensions, bounding_r: Number) -> Number {
    (bounding_r.powi(2) - dim.y.powi(2)).sqrt()
}
