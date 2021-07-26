use svg::node::element::{Circle, Group, Rectangle};

use flatpath::unit::{mm, Number};

pub struct AVModule {
    pub width: f32,
    pub height: f32,
    pub padding: f32,
    cy: f32,
}

impl AVModule {
    pub fn new(padding: f32) -> Self {
        let component_w = vec![12., 13., 12., 51.];
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
            Group2 {
                width: mm(12.),
                height: mm(12.),
                element: Group::new()
                    .add(
                        Circle::new()
                            .set("r", mm(3.))
                            .set("cx", mm(6.))
                            .set("cy", self.cy),
                    )
                    .add(
                        Circle::new()
                            .set("r", mm(6.))
                            .set("cx", mm(6.))
                            .set("cy", self.cy),
                    ),
            },
            // switch
            Group2 {
                width: mm(13.),
                height: mm(12.),
                element: Group::new()
                    .add(
                        Circle::new()
                            .set("r", mm(3.))
                            .set("cx", mm(13.) / 2.)
                            .set("cy", self.cy),
                    )
                    .add(
                        Rectangle::new()
                            .set("width", mm(13.))
                            .set("height", mm(12.)),
                    ),
            },
            // piezo
            Group2 {
                width: mm(12.),
                height: mm(12.),
                element: Group::new().add(
                    Circle::new()
                        .set("r", mm(6.))
                        .set("cx", mm(6.))
                        .set("cy", self.cy),
                ),
            },
            // LEDs
            Group2 {
                width: mm(51.),
                height: mm(10.),
                element: Group::new().add(
                    Rectangle::new()
                        .set("width", mm(51.))
                        .set("height", mm(10.))
                        .set("y", (self.height - mm(10.)) / 2.),
                ),
            },
        ];

        let mut x = self.padding;
        for c in components {
            g = g.add(c.element.set("transform", format!("translate({}, 0)", x)));
            x += c.width;
            x += self.padding;
        }

        g
    }
}

struct Group2 {
    width: Number,
    height: Number,
    element: Group,
}
