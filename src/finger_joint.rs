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
    num_fingers: usize, // is there a better word for this? number of fingers total b/t a/b
    radius: Number,
    // biased
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
    pub fn num_fingers(mut self, n: usize) -> FingerJointBuilder {
        self.num_fingers = n;
        self
    }
    pub fn radius(mut self, radius: Number) -> FingerJointBuilder {
        self.radius = radius;
        self
    }
    fn build_part(&self, part: FingerJointPart) -> PathBuilder {
        let (y1, y2) = match part {
            FingerJointPart::A => (self.height, 0.),
            FingerJointPart::B => (0., self.height),
        };

        let finger_length = self.width / (self.num_fingers as Number);

        let mut pb = PathBuilder::new();
        for i in 0..self.num_fingers {
            let x1 = finger_length * i as Number;
            let x2 = x1 + finger_length;
            if i & 1 == 0 {
                // even
                pb = pb.add(Point(x1, y1)).add(Point(x2, y1));
            } else {
                // odd
                pb = pb.add(Point(x1, y2)).add(Point(x2, y2));
            }
        }

        pb
    }
    // TODO pass in y1/y2 for a/b parts
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
    pub fn a(self) -> PathBuilder {
        self.a
    }
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
    fn finger_joint() {
        let joint = FingerJoint::builder()
            .width(100.)
            .height(10.)
            .num_fingers(4)
            .build();
        let (a, b) = joint.parts();
        assert_eq!(
            a.build(),
            "M0,10 L25,10 L25,0 L50,0 L50,10 L75,10 L75,0 L100,0 "
        );
        assert_eq!(
            b.build(),
            "M0,0 L25,0 L25,10 L50,10 L50,0 L75,0 L75,10 L100,10 "
        );
    }
}
