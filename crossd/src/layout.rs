#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Length {
    /// Maximum length.
    Max,
    /// Minimum length.
    Min,
    /// Pixels.
    Px(u32),
    /// Percent.
    Pc(u32),
}
