use std::collections::HashMap;
use std::io;

mod frame_brain_box;
mod keychain;
//mod lattice;
mod plant_ladder;
//mod quefrency_60_case;
mod small_rack;

pub trait Project {
    fn generate(&self) -> svg::Document;
}

type Projects<'a> = HashMap<&'a str, Box<dyn Project + 'static>>;

pub fn projects<'a>() -> Projects<'a> {
    let mut projects: Projects<'a> = HashMap::new();
    projects.insert("plant-ladder", Box::new(plant_ladder::PlantLadder));
    /*
    projects.insert(
        "quefrency-60-case",
        Box::new(quefrency_60_case::KeyboardCase),
    );
    */
    projects.insert("small-rack", Box::new(small_rack::SmallRack));
    //projects.insert("lattice", Box::new(lattice::Lattice));
    projects.insert("keychain", Box::new(keychain::Keychain));
    projects.insert("frame_brain_box", Box::new(frame_brain_box::BrainBox));
    projects
}

// TODO output path
pub fn save(project: &dyn Project) -> io::Result<()> {
    let doc = project.generate();
    svg::save("image.svg", &doc)
}
