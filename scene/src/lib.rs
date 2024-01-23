#![feature(const_fn_floating_point_arithmetic)]

use crossd_math::Trans2;

pub use self::color::*;
pub use self::element::*;
pub use self::path::*;

mod color;
mod element;
mod path;

#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    elements: Vec<Element>,
}

impl Scene {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn fill(&mut self, path: impl Into<Path>, fill: Fill) {
        self.elements.push(Element::Fill { path: path.into(), fill })
    }

    pub fn stroke(&mut self, path: impl Into<Path>, stroke: Stroke) {
        self.elements.push(Element::Stroke { path: path.into(), stroke })
    }

    pub fn group(&mut self, trans: Trans2, mut f: impl FnMut(&mut Scene)) {
        let mut scene = Scene::new();

        f(&mut scene);

        self.elements.push(Element::Group { trans, members: scene.elements });
    }

    pub fn clear(&mut self) {
        self.elements.clear()
    }
}
