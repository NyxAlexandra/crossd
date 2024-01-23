use crossd_math::{vec2, Point2, Rect, Size2, Trans2};

/// A bezier path represented by it's imperative [verbs](Verb).
///
/// See-also: [`PathBuilder`].
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub(crate) verbs: Vec<PathVerb>,
    pub(crate) points: Vec<Point2>,
}

/// Builder interface for [`Path`].
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq)]
pub struct PathBuilder {
    path: Path,
}

/// Components that make up a [`Path`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathVerb {
    Move,
    Line,
    Quad,
    Cubic,
    Close,
}

impl Path {
    /// Create a new path with no operations.
    ///
    /// ```
    /// # use crossd_graphics::scene::Path;
    /// #
    /// let mut path = Path::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self { verbs: Vec::new(), points: Vec::new() }
    }

    /// Create a builder of a path.
    ///
    /// ```
    /// # use crossd_graphics::scene::Path;
    /// #
    /// let path = Path::builder().build();
    ///
    /// assert_eq!(path, Path::new());
    /// ```
    #[must_use]
    pub const fn builder() -> PathBuilder {
        PathBuilder::new()
    }

    pub fn move_to(&mut self, to: Point2) {
        self.verbs.push(PathVerb::Move);
        self.points.push(to);
    }

    pub fn line_to(&mut self, to: Point2) {
        self.verbs.push(PathVerb::Line);
        self.points.push(to);
    }

    pub fn quad_to(&mut self, to: Point2, ctrl: Point2) {
        self.verbs.push(PathVerb::Quad);
        self.points.push(to);
        self.points.push(ctrl);
    }

    pub fn cubic_to(&mut self, to: Point2, ctrl: [Point2; 2]) {
        self.verbs.push(PathVerb::Cubic);
        self.points.push(to);
        self.points.extend(ctrl);
    }

    pub fn close(&mut self) {
        self.verbs.push(PathVerb::Close);
    }

    #[must_use]
    pub fn pop(&mut self) -> Option<PathVerb> {
        self.verbs.pop()
    }

    pub fn transform(&mut self, trans: Trans2) {
        for point in &mut self.points {
            *point = trans * *point;
        }
    }

    #[must_use]
    pub fn verbs(&self) -> &[PathVerb] {
        &self.verbs
    }

    #[must_use]
    pub fn points(&self) -> &[Point2] {
        &self.points
    }

    #[must_use]
    pub fn transformed(mut self, trans: Trans2) -> Self {
        self.transform(trans);

        self
    }

    #[must_use]
    pub fn into_builder(self) -> PathBuilder {
        PathBuilder { path: self }
    }
}

impl PathBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self { path: Path::new() }
    }

    #[must_use]
    pub fn move_to(mut self, point: Point2) -> Self {
        self.path.move_to(point);

        self
    }

    #[must_use]
    pub fn line_to(mut self, point: Point2) -> Self {
        self.path.line_to(point);

        self
    }

    #[must_use]
    pub fn quad_to(mut self, to: Point2, ctrl: Point2) -> Self {
        self.path.quad_to(to, ctrl);

        self
    }

    #[must_use]
    pub fn cubic_to(mut self, to: Point2, ctrl: [Point2; 2]) -> Self {
        self.path.cubic_to(to, ctrl);

        self
    }

    #[must_use]
    pub fn close(mut self) -> Self {
        self.path.close();

        self
    }

    /// Consume the builder, returning the built path.
    ///
    /// ```
    /// # use crossd_graphics::scene::Path;
    /// # use crossd_graphics::geometry::point2;
    /// #
    /// let point = point2(100.0, 100.0);
    ///
    /// let builder = Path::builder().move_to(point).build();
    /// let mut imperative = Path::new();
    ///
    /// imperative.move_to(point);
    ///
    /// assert_eq!(builder, imperative);
    /// ```
    #[must_use]
    pub fn build(self) -> Path {
        self.path
    }

    /// Close the path and consume the builder, returning the built path.
    ///
    /// ```
    /// # use crossd_graphics::scene::{Path, Verb};
    /// #
    /// let path = Path::builder().finish();
    ///
    /// assert_eq!(path.ops().last(), Some(Verb::Close));
    /// ```
    #[must_use]
    pub fn finish(self) -> Path {
        self.close().build()
    }
}

impl<T: Into<f32>> From<Rect<T>> for Path {
    fn from(rect: Rect<T>) -> Self {
        let Size2 { w, h } = rect.size.map(Into::into);

        // nw - ne
        // |     |
        // sw - se

        let nw = rect.loc.map(Into::into);
        let ne = nw + vec2(w, 0.0);
        let se = ne + vec2(0.0, h);
        let sw = se - vec2(w, 0.0);

        Path::builder()
            .move_to(nw)
            .line_to(ne)
            .line_to(se)
            .line_to(sw)
            .line_to(nw)
            .finish()
    }
}
