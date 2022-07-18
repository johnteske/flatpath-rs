pub enum Command {
    MoveTo(Point),
    LineTo(Point),
    LineToWithRadius(Point, Radius),
    Close,
}
impl Command {
    fn point(&self) -> Option<&Point> {
        match self {
            Self::MoveTo(point) => Some(point),
            Self::LineTo(point) => Some(point),
            Self::LineToWithRadius(point, ..) => Some(point),
            Self::Close => None,
        }
    }
}

pub struct Point(f32, f32);
impl Point {
    fn to_svg_string(&self) -> String {
        format!("{},{}", self.0, self.1)
    }
}
impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Point(x, y)
    }
}

type Radius = f32;

#[derive(Default)]
pub struct PathBuilder(Vec<Command>);

impl PathBuilder {
    pub fn new() -> Self {
        PathBuilder::default()
    }

    pub fn move_to(mut self, point: impl Into<Point>) -> Self {
        self.0.push(Command::MoveTo(point.into()));
        self
    }

    pub fn line_to(mut self, point: impl Into<Point>) -> Self {
        self.0.push(Command::LineTo(point.into()));
        self
    }

    pub fn line_to_r(mut self, point: impl Into<Point>, radius: Radius) -> Self {
        assert!(radius >= 0., "radius must be > 0");
        self.0.push(Command::LineToWithRadius(point.into(), radius));
        self
    }

    pub fn close(mut self) -> Self {
        self.0.push(Command::Close);
        self
    }

    pub fn build(&self) -> String {
        let mut data = String::new();

        for (i, cmd) in self.0.iter().enumerate() {
            if i != 0 {
                data.push_str(&format!(" "));
            }
            match cmd {
                Command::MoveTo(point) => {
                    data.push_str(&format!("M{}", point.to_svg_string()));
                }
                Command::LineTo(point) => {
                    data.push_str(&format!("L{}", point.to_svg_string()));
                }
                // TODO for now, assume this can't be the first command
                Command::LineToWithRadius(point, radius) => {
                    const TODO: &str =
                        "LineToWithRadius as first or last point not (yet) supported";
                    let previous_point = self.0[i - 1].point().expect(TODO);
                    let next_point = self.0[i + 1].point().expect(TODO);

                    let prev_point = point_along_line(&point, previous_point, *radius);
                    data.push_str(&format!("L{} ", prev_point.to_svg_string()));

                    let next_point = point_along_line(&point, next_point, *radius);
                    data.push_str(&format!(
                        "Q{} {}",
                        point.to_svg_string(),
                        next_point.to_svg_string()
                    ));
                }
                Command::Close => {
                    data.push_str(&format!("Z"));
                }
            }
        }

        data
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
            .move_to((0., 0.))
            .line_to_r(Point(50., 0.), 4.)
            .line_to_r(Point(50., 50.), 8.)
            .line_to(Point(0., 50.))
            .close()
            .build();
        let expected = "M0,0 L46,0 Q50,0 50,4 L50,42 Q50,50 42,50 L0,50 Z";
        assert_eq!(actual, expected);
    }

    /*
    #[test]
    fn with_start_radius() {
        let actual = PathBuilder::new()
            .line_to_r(Point(0., 0.), 4.)
            .line_to_r(Point(50., 0.), 4.)
            .line_to_r(Point(50., 50.), 8.)
            .line_to(Point(0., 50.))
            .close()
            .build();
        let expected = "M0,4 Q0,0 4,0 L46,0 Q50,0 50,4 L50,42 Q50,50 42,50 L0,50 Z";
        assert_eq!(actual, expected);
    }

    #[test]
    fn with_end_radius() {
        let actual = PathBuilder::new()
            .line_to(Point(0., 0.))
            .line_to_r(Point(50., 0.), 4.)
            .line_to_r(Point(50., 50.), 8.)
            .line_to_r(Point(0., 50.), 4.)
            .close()
            .build();
        let expected = "M0,0 L46,0 Q50,0 50,4 L50,42 Q50,50 42,50 L4,50 Q0,50 0,46 Z";
        assert_eq!(actual, expected);
    }

    #[test]
    fn with_start_end_radii() {
        let actual = PathBuilder::new()
            .line_to_r(Point(0., 0.), 4.)
            .line_to_r(Point(50., 0.), 4.)
            .line_to_r(Point(50., 50.), 8.)
            .line_to_r(Point(0., 50.), 4.)
            .close()
            .build();
        let expected = "M0,4 Q0,0 4,0 L46,0 Q50,0 50,4 L50,42 Q50,50 42,50 L4,50 Q0,50 0,46 Z";
        assert_eq!(actual, expected);
    }
    */
}
