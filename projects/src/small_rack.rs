use super::Project;
use svg::node::element::{Circle, Group, Rectangle};
use svg::Document;

use flatpath::unit::{mm, Number};

struct Dimensions {
    x: Number,
    y: Number,
}

struct Tool {
    /// diameter of the part of the tool to be held in rack
    shaft_diameter: Number,
    /// diameter of the part of the tool above the hole
    outer_diameter: Number,
    /// length of the part of the tool to be held in the rack
    length: Number,
}

impl Tool {
    pub fn new(shaft_diameter: Number, outer_diameter: Number, length: Number) -> Self {
        Tool {
            shaft_diameter,
            outer_diameter,
            length,
        }
    }

    pub fn max_diameter(&self) -> Number {
        self.shaft_diameter.max(self.outer_diameter)
    }

    pub fn to_svg(&self) -> (Group, Dimensions) {
        let max_d = self.max_diameter();
        let r = max_d * 0.5;
        let g = Group::new()
            // cut hole
            .add(
                Circle::new()
                    .set("r", self.shaft_diameter * 0.5)
                    .set("cx", r)
                    .set("cy", r),
            )
            // outline
            .add(
                Circle::new()
                    .set("r", self.outer_diameter * 0.5)
                    .set("cx", r)
                    .set("cy", r)
                    .set("stroke", "red"),
            );
        let dimensions = Dimensions { x: max_d, y: max_d };
        (g, dimensions)
    }
}

pub struct SmallRack;
impl Project for SmallRack {
    fn generate(&self) -> Document {
        // parameters
        let t = mm(3.);
        let padding = t;
        let mut tools = vec![
            Tool::new(mm(3.), mm(5.), 0.),
            Tool::new(mm(3.), mm(7.), 0.),
            Tool::new(mm(3.), mm(5.), 0.),
            Tool::new(mm(2.), mm(3.), 0.),
            Tool::new(mm(2.), mm(4.), 0.),
            Tool::new(mm(2.), mm(3.), 0.),
        ];
        tools.sort_by(|a, b| b.max_diameter().partial_cmp(&a.max_diameter()).unwrap());
        let rows = tools.chunks(3);

        let mut g = Group::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1);

        let mut tools_g = Group::new();
        let mut max_x = 0_f32;
        let mut x = 0.;
        let mut y = 0.;
        for r in rows {
            for t in r {
                let (tool_g, dim) = t.to_svg();
                tools_g = tools_g.add(tool_g.set("transform", format!("translate({},{})", x, y)));
                x += dim.x + padding;
            }
            max_x = max_x.max(x);
            x = 0.;
            // assume the first is the largest
            y += r.first().unwrap().max_diameter() + padding;
        }
        max_x -= padding;
        tools_g = tools_g.set("transform", format!("translate({}, {})", padding, padding));
        g = g.add(tools_g);

        g = g.add(
            Rectangle::new()
                .set("width", t + max_x + t)
                .set("height", t + y + t)
                .set("rx", t)
                .set("ry", t),
        );

        Document::new().set("viewBox", (0, 0, 999, 999)).add(g)
    }
}
