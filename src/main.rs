mod builder;
mod projects;
mod units;

fn main() {
    let doc = projects::quefrency_60_case::project();
    svg::save("image.svg", &doc).unwrap();
    println!("Hello, world!");
}
