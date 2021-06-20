use svg::node::element::{Circle, Group, Rectangle};

use crate::units::mm;

pub struct AVModule {
    pub width: f32,
    pub height: f32,
    pub padding: f32,
    cy: f32,
}

impl AVModule {
    pub fn new(padding: f32) -> Self {
        let component_w = vec![6., 6., 12., 51.];
        let sum: f32 = mm(component_w.iter().sum());
        let total_padding = (component_w.len() + 1) as f32 * padding;
        let width = sum + total_padding;

        let height = mm(12.);

        AVModule {
            width,
            height,
            padding,
            cy: height / 2.,
        }
    }

    pub fn render(self) -> Group {
        let mut g = Group::new();

        // module outline
        g = g.add(
            Rectangle::new()
                .set("width", self.width)
                .set("height", self.height)
                .set("stroke", "cyan"),
        );

        // TODO automatically distribute
        // set each component, that has its own x?
        // or loop through and inc x?

        let components = vec![
            // RCA
            mm(3.),
            mm(3.),
            mm(6.),
        ];

        let mut x = self.padding;
        for r in components {
            x += r;
            g = g.add(
                Circle::new()
                    .set("r", r)
                    .set("cy", self.cy)
                    .set("transform", format!("translate({}, 0)", x)),
            );
            x += r + self.padding;
        }

        // LEDs
        g = g.add(
            Rectangle::new()
                .set("width", mm(51.))
                .set("height", mm(10.))
                .set("y", (self.height - mm(10.)) / 2.)
                .set("transform", format!("translate({}, 0)", x)),
        );

        g
    }
}
