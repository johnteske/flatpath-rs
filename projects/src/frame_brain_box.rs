//! Box for Samsung's "The Frame" brain
//!
//! Design goals:
//! - sits on floor, cables to the side
//! - top is covered to prevent dust
//! - comes apart easily to allow access to box

use super::Project;
use svg::node::element::{Circle, Group, Rectangle};
use svg::Document;

use flatpath::unit::{inches, mm};

pub struct BrainBox;
impl Project for BrainBox {
    fn generate(&self) -> Document {
        // parameters
        let t = mm(3.);
        let width = inches(3.75);
        let height = inches(9.);
        let depth = inches(1.5);

        let mut g = Group::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "1");

        let top_bottom = Rectangle::new().set("width", width).set("height", depth);

        let left = Rectangle::new().set("width", depth).set("height", height);
        let right = Rectangle::new().set("width", depth).set("height", height);

        let front_back = Rectangle::new().set("width", width).set("height", height);

        for side in [top_bottom, left, right, front_back] {
            g = g.add(side);
        }

        Document::new().set("viewBox", (0, 0, 999, 999)).add(g)
    }
}
