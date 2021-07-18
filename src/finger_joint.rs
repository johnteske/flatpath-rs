use crate::builder::{BuilderPoint, PathBuilder, Point};
use crate::units::Number;

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
    pub fn build(&self) -> FingerJoint {
        let finger_length = self.width / (self.num_fingers as Number);
        let mut a = PathBuilder::new();
        for i in 0..self.num_fingers {
            let x1 = finger_length * i as Number;
            let x2 = x1 + finger_length;
            if i & 1 == 0 {
                // even
                a = a.add(Point(x1, self.height)).add(Point(x2, self.height));
            } else {
                // odd
                a = a.add(Point(x1, 0.)).add(Point(x2, 0.));
            }
        }
        FingerJoint {
        a
        //b:
        }
    }
}

struct FingerJoint {
    a: PathBuilder,
    // b: PathBuilder
    // width, height
}

impl FingerJoint {
    pub fn builder() -> FingerJointBuilder {
        FingerJointBuilder::new()
    }
    pub fn a(self) -> PathBuilder {
        self.a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finger_joint() {
        let joint = FingerJoint::builder()
            .width(100.)
            .height(10.)
            .num_fingers(3)
            .build();
        let a = joint.a();
        assert_eq!(a.build(), "DATA");
    }
}
