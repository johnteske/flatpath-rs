mod builder;
mod projects;
mod units;

mod element; // TODO this should be its own crate

fn main() {
    let doc = projects::quefrency_60_case::project();
    svg::save("image.svg", &doc).unwrap();
    println!("Hello, world!");
}
