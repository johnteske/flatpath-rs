use crate::builder::{PathBuilder, Point};
use crate::units::Number;

// terminology
// pin/socket
//
// half pin, in the case the first/last is.. half

// AKA box joint
// TODO this could also handle mortise and tenon joints?
//
// TODO option to set bias
#[derive(Default)]
struct FingerJointBuilder {
    width: Number,
    height: Number,
    n_fingers: usize, // is there a better word for this? number of fingers total b/t a/b
    radius: Number,
    // biased: bool,
}

impl FingerJointBuilder {
    pub fn new() -> Self {
        FingerJointBuilder::default()
    }
    pub fn width(mut self, width: Number) -> FingerJointBuilder {
        self.width = width;
        self
    }
    pub fn height(mut self, height: Number) -> FingerJointBuilder {
        self.height = height;
        self
    }
    pub fn n_fingers(mut self, n: usize) -> FingerJointBuilder {
        self.n_fingers = n;
        self
    }
    pub fn radius(mut self, radius: Number) -> FingerJointBuilder {
        self.radius = radius;
        self
    }
    fn build_part(&self, part: FingerJointPart) -> PathBuilder {
        let (y0, y1) = match part {
            FingerJointPart::A => (0., -self.height),
            FingerJointPart::B => (0., self.height),
        };

        let finger_length = self.width / (self.n_fingers as Number);

        let mut pb = PathBuilder::new();
        for i in 0..self.n_fingers {
            let x0 = finger_length * i as Number;
            let x1 = x0 + finger_length;
            if i == 0 {
                pb = pb.add(Point(x0, y0));
            }
            // TODO this is backwards for part B
            if i & 1 == 0 {
                // even
                pb = pb.add(Point(x0, y1)).add(Point(x1, y1));
            } else {
                // odd
                pb = pb.add(Point(x0, y0)).add(Point(x1, y0));
            }

            // TODO if doesn't at at y0, add a point
        }

        pb
    }
    pub fn build(&self) -> FingerJoint {
        let a = self.build_part(FingerJointPart::A);
        let b = self.build_part(FingerJointPart::B);

        FingerJoint { a, b }
    }
}

struct FingerJoint {
    a: PathBuilder,
    b: PathBuilder,
    // width, height
}

impl FingerJoint {
    pub fn builder() -> FingerJointBuilder {
        FingerJointBuilder::new()
    }
    /// Fingers protruding "up"
    pub fn a(self) -> PathBuilder {
        self.a
    }
    /// Fingers protruding "down"
    pub fn b(self) -> PathBuilder {
        self.b
    }
    pub fn parts(self) -> (PathBuilder, PathBuilder) {
        (self.a, self.b)
    }
}

enum FingerJointPart {
    A,
    B,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finger_joint_even() {
        let joint = FingerJoint::builder()
            .width(100.)
            .height(10.)
            .n_fingers(4)
            .build();
        let (a, b) = joint.parts();
        assert_eq!(
            a.build(),
            "M0,0 L0,-10 L25,-10 L25,0 L50,0 L50,-10 L75,-10 L75,0 L100,0 "
        );
        // assert_eq!(
        //   b.build(),
        //   "M0,0 L25,0 L25,10 L50,10 L50,0 L75,0 L75,10 L100,10 "
        // );
    }
    #[test]
    fn finger_joint_odd() {
        let joint = FingerJoint::builder()
            .width(60.)
            .height(10.)
            .n_fingers(3)
            .build();
        let (a, b) = joint.parts();
        assert_eq!(
            a.build(),
            "M0,0 L0,-10 L20,-10 L20,0 L40,0 L40,-10 L60,-10 L60,0 " // TODO add logic to put point at y0
        );
        // assert_eq!(
        //   b.build(),
        //   "M0,0 L25,0 L25,10 L50,10 L50,0 L75,0 L75,10 L100,10 "
        // );
    }
}
