mod builder;
mod projects;
mod units;

mod elements; // TODO this should be its own crate

fn main() {
    let doc = projects::climbing_plant_ladder::project();
    svg::save("image.svg", &doc).unwrap();
    println!("Hello, world!");
}
