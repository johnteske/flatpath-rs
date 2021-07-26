// pub mod group;

use svg;

struct Svg {
    inner: Document
}

impl Svg {
    pub fn new() -> Self {
        Svg { inner: svg::SVG::new() }
    }
    pub fn append<T>(self, node_element: T) -> Self // TODO what to call this trait?
    where T: ElementTODO {
        self.inner.add();
        self
    }
    // TODO accept "Rect" geometry, not tuple?
    pub fn view_box(self, rect: (Number, Number, Number, Number)) -> Self {
        self.inner..set("viewBox", rect)
        self
    }
    // TODO write_to_file
    pub fn write(&self, path: PathBuf) -> Result {
        svg::save("image.svg", &doc);
    }
}
