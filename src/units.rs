static DPI: f32 = 96.;

pub fn inch(a: f32) -> f32 {
    a * DPI
}

pub fn mm(a: f32) -> f32 {
    (a * DPI) / 25.4
}
