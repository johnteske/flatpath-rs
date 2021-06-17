//struct Point(f32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    radius: Option<f32>,
}

// add absolute points, with optional radius,
// returns SVG data path
#[derive(Default)]
struct PathBuilder(Vec<Point>);

impl PathBuilder {
    fn new() -> Self {
        PathBuilder::default()
    }

    // TODO is point the right word?
    fn add(mut self, point: (f32, f32), radius: f32) -> Self {
        // TODO what about handling negative radii?
        // this currently ignores
        if radius > 0.0 {
            self.0.push(Point {
                x: point.0,
                y: point.1,
                radius: Some(radius),
            });
            return self;
        }

        self.0.push(Point {
            x: point.0,
            y: point.1,
            radius: None,
        });
        self
    }

    // TODO first point should always be M (move)
    // and if the first point is rounded, calculate b/t last point
    fn build(&self) -> String {
        let mut data = String::new();

        &self.0.iter().enumerate().for_each(|(i, p)| {
            match p.radius {
                None => {
                    let command = match i {
                        0 => "M",
                        _ => "L",
                    };

                    data.push_str(&format!("{}{},{} ", command, p.x, p.y));
                }
                Some(radius) => {
                    if i != 0 && (i != self.0.len() - 1) {
                        let previous_point = &self.0[i - 1];
                        let pt = point_along_line((p.x, p.y), (previous_point.x, previous_point.y), radius);
                        data.push_str(&format!("L{:?} ", pt));

                        let next_point = &self.0[i + 1];
                        let pt2 = point_along_line((p.x, p.y), (next_point.x, next_point.y), radius);
                        data.push_str(&format!("A {:?},{:?} 0 0 1 {:?} ", radius, radius, pt2));
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

fn point_along_line(p1: (f32, f32), p2: (f32, f32), dt: f32) -> (f32, f32) {
    let x0 = p1.0;
    let y0 = p1.1;
    let x1 = p2.0;
    let y1 = p2.1;
    let d = ((x1 - x0).powf(2.) + (y1 - y0).powf(2.)).sqrt();
    let t = dt / d;
    let xt = ((1. - t) * x0) + (t * x1);
    let yt = ((1. - t) * y0) + (t * y1);
    (xt, yt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = PathBuilder::new()
            .add((0., 0.), 0.)
            .add((50., 0.), 4.)
            .add((50., 50.), 8.)
            .add((0., 50.), 0.)
            .close();
        let expected = "L20,20 L50,50 Z";
        assert_eq!(actual, expected);
    }
}
