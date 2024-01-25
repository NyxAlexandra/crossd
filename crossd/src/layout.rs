use crossd_math::Size2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dim2 {
    pub w: Length,
    pub h: Length,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Length {
    /// Fill maximum amount of space.
    Fill,
    /// Fill minimum amount of space.
    Shrink,
    /// Fill an amount of pixels.
    Px(u32),
}

impl Dim2 {
    pub fn resolve(self, bounds: Size2) -> Size2 {
        todo!()
    }
}
