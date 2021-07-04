static DPI: f32 = 96.;

pub type Number = f32;

pub fn inch(a: Number) -> Number {
    a * DPI
}

pub fn mm(a: Number) -> Number {
    (a * DPI) / 25.4
}
