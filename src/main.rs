mod builder;
mod projects;
mod units;

mod elements; // TODO this should be its own crate
mod finger_joint;

fn main() {
    let doc = projects::plant_ladder::project();
    svg::save("image.svg", &doc).unwrap();
    println!("Hello, world!");
}
