use crate::units::mm;

pub struct AVModule {
    pub height: f32,
    pub width: f32,
}

impl AVModule {
    pub fn new(padding: f32) -> Self {
        let component_w = vec![6., 6., 12., 51.];
        let sum: f32 = component_w.iter().sum();

        // add padding between componets
        let width = mm(sum + ((component_w.len() - 1) as f32 * padding));

        AVModule {
            height: padding + mm(12.) + padding,
            width,
        }
    }
}
