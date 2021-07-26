use std::fmt;

type Radius = f32;

struct Point(f32, f32);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct BuilderPoint {
    x: f32,
    y: f32,
    radius: Option<f32>,
}

impl BuilderPoint {
    fn point(&self) -> Point {
        Point(self.x, self.y)
    }
}

// add absolute points, with optional radius,
// returns SVG data path
#[derive(Default)]
pub struct PathBuilder(Vec<BuilderPoint>);

impl PathBuilder {
    pub fn new() -> Self {
        PathBuilder::default()
    }

    pub fn add(self, point: (f32, f32)) -> Self {
        self.add_r(point, 0.)
    }

    pub fn add_r(mut self, point: (f32, f32), radius: Radius) -> Self {
        let r = match radius {
            r if r == 0. => None,
            r if r > 0. => Some(r),
            _ => panic!("radius must be >= 0."),
        };

        self.0.push(BuilderPoint {
            x: point.0,
            y: point.1,
            radius: r,
        });

        self
    }

    pub fn map(self, f: impl FnMut(&BuilderPoint) -> BuilderPoint) -> Self {
        let new_points = self.0.iter().map(f).collect::<Vec<BuilderPoint>>();
        PathBuilder(new_points)
    }

    fn build(&self) -> String {
        let mut data = String::new();

        self.0.iter().enumerate().for_each(|(i, bpoint)| {
            let point = bpoint.point();

            let command = if i == 0 { "M" } else { "L" };

            match bpoint.radius {
                None => {
                    data.push_str(&format!("{}{} ", command, point));
                }
                Some(radius) => {
                    let prev_index = if i == 0 { self.0.len() - 1 } else { i - 1 };
                    let next_index = if i == self.0.len() - 1 { 0 } else { i + 1 };

                    let prev_bpoint = &self.0[prev_index];
                    let prev_point = point_along_line(&point, &prev_bpoint.point(), radius);
                    data.push_str(&format!("{}{} ", command, prev_point));

                    let next_bpoint = &self.0[next_index];
                    let next_point = point_along_line(&point, &next_bpoint.point(), radius);
                    data.push_str(&format!("Q{} {} ", point, next_point));
                }
            }
        });

        data
    }

    // build + Z
    pub fn close(&self) -> String {
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
    fn without_start_end_radii() {
        let actual = PathBuilder::new()
            .add((0., 0.))
            .add_r((50., 0.), 4.)
            .add_r((50., 50.), 8.)
            .add((0., 50.))
            .close();
        let expected = "M0,0 L46,0 Q50,0 50,4 L50,42 Q50,50 42,50 L0,50 Z";
        assert_eq!(actual, expected);
    }

    #[test]
    fn with_start_radius() {
        let actual = PathBuilder::new()
            .add_r((0., 0.), 4.)
            .add_r((50., 0.), 4.)
            .add_r((50., 50.), 8.)
            .add((0., 50.))
            .close();
        let expected = "M0,4 Q0,0 4,0 L46,0 Q50,0 50,4 L50,42 Q50,50 42,50 L0,50 Z";
        assert_eq!(actual, expected);
    }

    #[test]
    fn with_end_radius() {
        let actual = PathBuilder::new()
            .add((0., 0.))
            .add_r((50., 0.), 4.)
            .add_r((50., 50.), 8.)
            .add_r((0., 50.), 4.)
            .close();
        let expected = "M0,0 L46,0 Q50,0 50,4 L50,42 Q50,50 42,50 L4,50 Q0,50 0,46 Z";
        assert_eq!(actual, expected);
    }

    #[test]
    fn with_start_end_radii() {
        let actual = PathBuilder::new()
            .add_r((0., 0.), 4.)
            .add_r((50., 0.), 4.)
            .add_r((50., 50.), 8.)
            .add_r((0., 50.), 4.)
            .close();
        let expected = "M0,4 Q0,0 4,0 L46,0 Q50,0 50,4 L50,42 Q50,50 42,50 L4,50 Q0,50 0,46 Z";
        assert_eq!(actual, expected);
    }

    #[test]
    fn map() {
        let actual = PathBuilder::new()
            .add_r((0., 0.), 4.)
            .add_r((50., 0.), 4.)
            .add_r((50., 50.), 8.)
            .add_r((0., 50.), 4.)
            .map(|p| BuilderPoint { x: p.x + 10., ..*p })
            .close();
        let expected = "M0,4 Q0,0 4,0 L46,0 Q50,0 50,4 L50,42 Q50,50 42,50 L4,50 Q0,50 0,46 Z";
        assert_eq!(actual, expected);
    }
}