use super::Project;
use svg::node::element::{Circle, Group, Rectangle};
use svg::Document;

use flatpath::unit::{inches, mm};

pub struct Keychain;
impl Project for Keychain {
    fn generate(&self) -> Document {
        // parameters
        let t = mm(3.);
        let width = inches(1.);
        let height = inches(2.);

        let mut g = Group::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1);

        g = g.add(
            Rectangle::new()
                .set("width", width)
                .set("height", height)
                .set("rx", t)
                .set("ry", t),
        );

        g = g.add(Circle::new().set("r", t / 2.).set(
            "transform",
            format!("translate({}, {})", width / 2., t * 1.5),
        ));

        Document::new().set("viewBox", (0, 0, 999, 999)).add(g)
    }
}
