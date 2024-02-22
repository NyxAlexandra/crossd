use std::{slice, vec};

pub use element::*;
pub use path::*;

use crate::{Mat3, Point2};

mod element;
mod path;

/// A vector scene.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Scene<T = f32> {
    elements: Vec<Element<T>>,
}

/// By-reference [`Iterator`] over [`Element`]s of a [`Scene`].
pub struct Iter<'a, T> {
    iter: slice::Iter<'a, Element<T>>,
}

/// By-mutable-reference [`Iterator`] over [`Element`]s of a [`Scene`].
pub struct IterMut<'a, T> {
    iter: slice::IterMut<'a, Element<T>>,
}

/// By-value [`Iterator`] over [`Element`]s of a [`Scene`].
pub struct IntoIter<T> {
    iter: vec::IntoIter<Element<T>>,
}

impl<T> Scene<T> {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn fill(&mut self, path: impl Into<Path<T>>, fill: &Fill) {
        self.elements.push(Element::Fill { path: path.into(), fill: fill.clone() });
    }

    pub fn stroke(&mut self, path: impl Into<Path<T>>, stroke: &Stroke) {
        self.elements.push(Element::Stroke { path: path.into(), stroke: stroke.clone() });
    }

    pub fn image(&mut self, point: impl Into<Point2<T>>, image: &Image) {
        self.elements.push(Element::Image { point: point.into(), image: image.clone() });
    }

    pub fn group(&mut self, trans: Mat3<T>, f: impl FnOnce(&mut Self)) {
        let mut scene = Self::new();

        f(&mut scene);

        self.elements.push(Element::Group { trans, scene });
    }
}

impl<T> Default for Scene<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> IntoIterator for &'a Scene<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a Element<T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { iter: self.elements.iter() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a Element<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> IntoIterator for &'a mut Scene<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut Element<T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut { iter: self.elements.iter_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut Element<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> IntoIterator for Scene<T> {
    type IntoIter = IntoIter<T>;
    type Item = Element<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { iter: self.elements.into_iter() }
    }
}

impl<'a, T> Iterator for IntoIter<T> {
    type Item = Element<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<T> FromIterator<Element<T>> for Scene<T> {
    fn from_iter<I: IntoIterator<Item = Element<T>>>(iter: I) -> Self {
        let elements = iter.into_iter().collect();

        Self { elements }
    }
}
