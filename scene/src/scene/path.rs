// implementation mostly stolen from [`tiny-skia`](https://docs.rs/tiny-skia).

use std::slice;

use crate::math::Point2;
use crate::Zero;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Path<T = f32> {
    points: Box<[Point2<T>]>,
    verbs: Box<[PathVerb]>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PathBuilder<T> {
    points: Vec<Point2<T>>,
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
}

/// The components of a [`Path`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PathElement<T> {
    MoveTo(Point2<T>),
    LineTo(Point2<T>),
    QuadTo { p: Point2<T>, c: Point2<T> },
    CubicTo { p: Point2<T>, c1: Point2<T>, c2: Point2<T> },
}

#[derive(Debug, Clone)]
pub struct PathElements<'a, T> {
    points: slice::Iter<'a, Point2<T>>,
    verbs: slice::Iter<'a, PathVerb>,
}

impl<T: Zero> Path<T> {
    pub fn builder() -> PathBuilder<T> {
        PathBuilder::new()
    }

    pub fn points(&self) -> &[Point2<T>] {
        &self.points
    }

    pub fn verbs(&self) -> &[PathVerb] {
        &self.verbs
    }

    pub fn elements(&self) -> PathElements<T> {
        PathElements { points: self.points.iter(), verbs: self.verbs.iter() }
    }

    pub fn into_builder(self) -> PathBuilder<T> {
        PathBuilder::from_path(self)
    }
}

impl<T: Zero> PathBuilder<T> {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            verbs: Vec::new(),
            move_required: true,
            last_point: None,
        }
    }

    pub fn from_path(path: Path<T>) -> Self {
        let Path { points, verbs } = path;

        PathBuilder {
            points: points.into(),
            verbs: verbs.into(),
            move_required: true,
            last_point: None,
        }
    }

    pub fn move_to(&mut self, point: impl Into<Point2<T>>) {
        self.move_required = false;
        self.last_point = Some(self.points.len());

        self.points.push(point.into());
    }

    pub fn line_to(&mut self, point: impl Into<Point2<T>>) {
        self.move_if_required();

        self.verbs.push(PathVerb::Line);
        self.points.push(point.into());
    }

    pub fn quad_to(&mut self, p: impl Into<Point2<T>>, c: impl Into<Point2<T>>) {
        self.move_if_required();

        self.verbs.push(PathVerb::Quad);
        self.points.extend([p.into(), c.into()]);
    }

    pub fn cubic_to(
        &mut self,
        p: impl Into<Point2<T>>,
        c1: impl Into<Point2<T>>,
        c2: impl Into<Point2<T>>,
    ) {
        self.move_if_required();

        self.verbs.push(PathVerb::Quad);
        self.points.extend([p.into(), c1.into(), c2.into()]);
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.verbs.clear();

        self.move_required = true;
        self.last_point = None;
    }

    pub fn build(self) -> Option<Path<T>> {
        if self.verbs.is_empty() || self.verbs.len() == 1 {
            return None;
        }

        Some(Path { points: self.points.into(), verbs: self.verbs.into() })
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

impl<'a, T: Zero> Iterator for PathElements<'a, T> {
    type Item = PathElement<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next =
            || self.points.next().copied().expect("all paths are guaranteed to be valid");

        self.verbs.next().map(|verb| match verb {
            PathVerb::Move => PathElement::MoveTo(next()),
            PathVerb::Line => PathElement::LineTo(next()),
            PathVerb::Quad => PathElement::QuadTo { p: next(), c: next() },
            PathVerb::Cubic => PathElement::CubicTo { p: next(), c1: next(), c2: next() },
        })
    }
}

impl<'a, T: Zero> IntoIterator for &'a Path<T> {
    type IntoIter = PathElements<'a, T>;
    type Item = PathElement<T>;

    fn into_iter(self) -> Self::IntoIter {
        {
            self.elements()
        }
    }
}
