use svg::node::element::Group as _Group;

use super::circle::Circle;
use super::BoundingBox;

enum Node {
    Circle(Circle),
}

#[derive(Default)]
struct Group {
    children: Vec<Box<dyn BoundingBox>>,
}

impl Group {
    fn new() -> Self {
        Group::default()
    }
    fn append<T>(&mut self, element: T)
    where
        T: 'static + BoundingBox,
    {
        self.children.push(Box::new(element));
    }
}

//impl BoundingBox for Group {
//    fn x(&self) -> Number {
//        &self.cx - &self.r
//    }
//    fn y(&self) -> Number {
//        &self.cy - &self.r
//    }
//    fn width(&self) -> Number {
//        &self.r * 2.0
//    }
//    fn height(&self) -> Number {
//        self.width()
//    }
//}

impl Into<_Group> for Group {
    fn into(self) -> _Group {
        let g = _Group::new();
        for child in self.children {
            //let c = *child;
            let c: Box<dyn BoundingBox> = *child.into(); // 
            g = g.add(c);
        }
        todo!("add all elements")
        // _Group::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::circle;

    #[test]
    fn group() {
        let c = circle::Circle::new();
        let mut g = Group::new();
        g.append(c);
        // assert_eq!(c.width(), 6.0);
        // assert_eq!(c.height(), 6.0);
        // assert_eq!(c.x(), -3.0);

        let g: _Group = g.into();
        assert_eq!(g.to_string(), r#"<circle cx="0" cy="0" r="3"/>"#);
    }
}
