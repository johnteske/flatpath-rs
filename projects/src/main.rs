use svg;

mod plant_ladder;
mod quefrency_60_case;

fn main() {
    let doc = plant_ladder::project();
    svg::save("image.svg", &doc).unwrap();
    println!("Hello, world!");
}
