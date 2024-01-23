#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Length {
    /// Fill maximum amount of space.
    Fill,
    /// Fill minimum amount of space.
    Shrink,
    /// Fill an amount of pixels.
    Px(u32),
}
