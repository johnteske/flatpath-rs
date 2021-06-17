use std::fmt;

struct Point(f32, f32);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

impl From<&BuilderPoint> for Point {
    fn from(bpoint: &BuilderPoint) -> Self {
        Point(bpoint.x, bpoint.y)
    }
}

type Radius = f32;

#[derive(Debug)]
struct BuilderPoint {
    x: f32,
    y: f32,
    radius: Option<f32>,
}

// add absolute points, with optional radius,
// returns SVG data path
#[derive(Default)]
struct PathBuilder(Vec<BuilderPoint>);

impl PathBuilder {
    fn new() -> Self {
        PathBuilder::default()
    }

    fn add(mut self, point: Point) -> Self {
        self.0.push(BuilderPoint {
            x: point.0,
            y: point.1,
            radius: None,
        });

        self
    }

    fn add_r(mut self, point: Point, radius: Radius) -> Self {
        if radius <= 0. {
            panic!("radius must be > 0");
        }

        self.0.push(BuilderPoint {
            x: point.0,
            y: point.1,
            radius: Some(radius),
        });

        self
    }

    fn build(&self) -> String {
        let mut data = String::new();

        &self.0.iter().enumerate().for_each(|(i, bpoint)| {
            let point = Point::from(bpoint);

            match bpoint.radius {
                None => {
                    let command = match i {
                        0 => "M",
                        _ => "L",
                    };

                    data.push_str(&format!("{}{} ", command, point));
                }
                Some(radius) => {
                    // TODO first point should always be M (move)
                    // and if the first point is rounded, calculate b/t last point
                    if i != 0 && (i != self.0.len() - 1) {
                        let prev_bpoint = &self.0[i - 1];
                        let pt = point_along_line(&point, &Point::from(prev_bpoint), radius);
                        data.push_str(&format!("L{} ", pt));

                        let next_bpoint = &self.0[i + 1];
                        let pt2 = point_along_line(&point, &Point::from(next_bpoint), radius);
                        data.push_str(&format!("A {},{} 0 0 1 {} ", radius, radius, pt2));
                    } else {
                        unimplemented!("first or last element with radius needs to wrap to calculate intermediate points");
                    }
                }
            }
        });

        data
    }

    // build + Z
    fn close(&self) -> String {
        format!("{}Z", &self.build())
    }
}

fn point_along_line(p0: &Point, p1: &Point, dt: f32) -> Point {
    let x0 = p0.0;
    let y0 = p0.1;
    let x1 = p1.0;
    let y1 = p1.1;

    let d = ((x1 - x0).powf(2.) + (y1 - y0).powf(2.)).sqrt();
    let t = dt / d;

    let xt = ((1. - t) * x0) + (t * x1);
    let yt = ((1. - t) * y0) + (t * y1);

    Point(xt, yt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = PathBuilder::new()
            .add(Point(0., 0.))
            .add_r(Point(50., 0.), 4.)
            .add_r(Point(50., 50.), 8.)
            .add(Point(0., 50.))
            .close();
        let expected = "L20,20 L50,50 Z";
        assert_eq!(actual, expected);
    }
}
