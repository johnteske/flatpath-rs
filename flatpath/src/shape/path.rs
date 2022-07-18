use crate::unit::{Number, PositiveNormalNumber};
use svg::node::element::Path;

pub enum Command {
    MoveTo(Point),
    LineTo(Point),
    LineToWithRadius(Point, PositiveNormalNumber),
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

pub struct Point(Number, Number);
impl Point {
    pub fn new(x: Number, y: Number) -> Self {
        Self(x, y)
    }
    pub fn x(&self) -> Number {
        self.0
    }
    pub fn y(&self) -> Number {
        self.1
    }
    fn to_svg_string(&self) -> String {
        format!("{},{}", self.0, self.1)
    }
}
impl From<(Number, Number)> for Point {
    fn from((x, y): (Number, Number)) -> Self {
        Point(x, y)
    }
}

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

    pub fn line_to_r(mut self, point: impl Into<Point>, radius: PositiveNormalNumber) -> Self {
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

                    let prev_point = point_along_line(&point, previous_point, radius.get());
                    data.push_str(&format!("L{} ", prev_point.to_svg_string()));

                    let next_point = point_along_line(&point, next_point, radius.get());
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

    pub fn to_path(&self) -> Path {
        Path::new().set("d", self.build())
    }

    pub fn map(&self, f: impl FnMut(&Command) -> Command) -> Self {
        self.0.iter().map(f).collect()
    }
}

impl std::iter::FromIterator<Command> for PathBuilder {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Command>,
    {
        let mut commands = Vec::new();

        for i in iter {
            commands.push(i);
        }

        PathBuilder(commands)
    }
}

fn point_along_line(p0: &Point, p1: &Point, dt: Number) -> Point {
    let Point(x0, y0) = p0;
    let Point(x1, y1) = p1;

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
            .line_to_r(Point(50., 0.), PositiveNormalNumber::new(4.).unwrap())
            .line_to_r(Point(50., 50.), PositiveNormalNumber::new(8.).unwrap())
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
