use std::collections::HashMap;
use std::io;
use svg;

mod plant_ladder;
mod quefrency_60_case;

pub trait Project {
    fn generate(&self) -> svg::Document;
}

type Projects<'a> = HashMap<&'a str, Box::<dyn Project + 'static>>;

pub fn projects<'a>() -> Projects<'a> {
    let mut projects: Projects<'a> = HashMap::new();
    projects.insert("plant-ladder", Box::new(plant_ladder::PlantLadder));
    projects.insert("quefrency-60-case", Box::new(quefrency_60_case::KeyboardCase));
    projects
}

// TODO output path
pub fn save(project: &dyn Project) -> io::Result<()> {
    let doc = project.generate(); 
    svg::save("image.svg", &doc)
}
