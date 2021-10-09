fn main() {
    let projects = projects::projects();
    // TODO choose
    let p = projects.get("plant-ladder").unwrap();
    projects::save(&**p).expect("error saving");
}
