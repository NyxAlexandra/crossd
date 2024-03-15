// implementation mostly stolen from [`tiny-skia`](https://docs.rs/tiny-skia).

use std::slice;

use crate::math::{Point2, Rect};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Path {
    points: Box<[Point2<f32>]>,
    verbs: Box<[PathVerb]>,
    bounds: Rect<f32>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PathBuilder {
    points: Vec<Point2<f32>>,
    verbs: Vec<PathVerb>,

    move_required: bool,
    last_point: Option<usize>,
}

/// Verbs that make up the components of a [`Path`].
///
/// See-also [`PathElement`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PathVerb {
    Move,
    Line,
    Quad,
    Cubic,
    Close,
}

/// The components of a [`Path`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PathElement {
    MoveTo(Point2<f32>),
    LineTo(Point2<f32>),
    QuadTo { p: Point2<f32>, c: Point2<f32> },
    CubicTo { p: Point2<f32>, c1: Point2<f32>, c2: Point2<f32> },
    Close,
}

#[derive(Debug, Clone)]
pub struct PathElements<'a> {
    points: slice::Iter<'a, Point2<f32>>,
    verbs: slice::Iter<'a, PathVerb>,
}

impl Path {
    pub fn new(elements: impl IntoIterator<Item = PathElement>) -> Option<Self> {
        let mut builder = Self::builder();

        for elem in elements {
            match elem {
                PathElement::MoveTo(p) => builder.move_to(p),
                PathElement::LineTo(p) => builder.line_to(p),
                PathElement::QuadTo { p, c } => builder.quad_to(p, c),
                PathElement::CubicTo { p, c1, c2 } => builder.cubic_to(p, c1, c2),
                PathElement::Close => builder.close(),
            }
        }

        builder.build()
    }

    pub fn builder() -> PathBuilder {
        PathBuilder::new()
    }

    pub fn points(&self) -> &[Point2<f32>] {
        &self.points
    }

    pub fn verbs(&self) -> &[PathVerb] {
        &self.verbs
    }

    pub fn bounds(&self) -> Rect<f32> {
        self.bounds
    }

    pub fn elements(&self) -> PathElements {
        PathElements { points: self.points.iter(), verbs: self.verbs.iter() }
    }

    pub fn into_builder(self) -> PathBuilder {
        PathBuilder::from_path(self)
    }
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            verbs: Vec::new(),
            move_required: true,
            last_point: None,
        }
    }

    pub fn from_path(path: Path) -> Self {
        let Path { points, verbs, .. } = path;

        PathBuilder {
            points: points.into(),
            verbs: verbs.into(),
            move_required: true,
            last_point: None,
        }
    }

    pub fn move_to(&mut self, point: impl Into<Point2<f32>>) {
        self.move_required = false;
        self.last_point = Some(self.points.len());

        self.points.push(point.into());
    }

    pub fn line_to(&mut self, point: impl Into<Point2<f32>>) {
        self.move_if_required();

        self.verbs.push(PathVerb::Line);
        self.points.push(point.into());
    }

    pub fn quad_to(&mut self, p: impl Into<Point2<f32>>, c: impl Into<Point2<f32>>) {
        self.move_if_required();

        self.verbs.push(PathVerb::Quad);
        self.points.extend([p.into(), c.into()]);
    }

    pub fn cubic_to(
        &mut self,
        p: impl Into<Point2<f32>>,
        c1: impl Into<Point2<f32>>,
        c2: impl Into<Point2<f32>>,
    ) {
        self.move_if_required();

        self.verbs.push(PathVerb::Quad);
        self.points.extend([p.into(), c1.into(), c2.into()]);
    }

    pub fn close(&mut self) {
        if !self.verbs.is_empty() {
            self.verbs.push(PathVerb::Close);
        }
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.verbs.clear();

        self.move_required = true;
        self.last_point = None;
    }

    pub fn build(mut self) -> Option<Path> {
        if self.verbs.is_empty() || self.verbs.len() == 1 {
            return None;
        }
        if self.verbs.last() != Some(&PathVerb::Close) {
            self.close();
        }

        let bounds = Rect::from_iter(&self.points);

        Some(Path { points: self.points.into(), verbs: self.verbs.into(), bounds })
    }

    fn move_if_required(&mut self) {
        if self.move_required {
            if let Some(idx) = self.last_point {
                self.move_to(self.points[idx]);
            } else {
                self.move_to(Point2::ZERO);
            }
        }
    }
}

impl<'a> Iterator for PathElements<'a> {
    type Item = PathElement;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next =
            || self.points.next().copied().expect("all paths are guaranteed to be valid");

        self.verbs.next().map(|verb| match verb {
            PathVerb::Move => PathElement::MoveTo(next()),
            PathVerb::Line => PathElement::LineTo(next()),
            PathVerb::Quad => PathElement::QuadTo { p: next(), c: next() },
            PathVerb::Cubic => PathElement::CubicTo { p: next(), c1: next(), c2: next() },
            PathVerb::Close => PathElement::Close,
        })
    }
}

impl<'a> IntoIterator for &'a Path {
    type IntoIter = PathElements<'a>;
    type Item = PathElement;

    fn into_iter(self) -> Self::IntoIter {
        {
            self.elements()
        }
    }
}
