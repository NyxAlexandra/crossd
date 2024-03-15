use crate::math::Size2;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub struct Layout {
    pub size: Size2<Length>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub struct Bounds {
    pub min: Size2<u32>,
    pub max: Size2<u32>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Length {
    /// Maximum length.
    #[default]
    Max,
    /// Minimum length.
    Min,
    /// Percent.
    Pc(u32),
    /// Pixels.
    Px(u32),
}

impl Layout {
    pub fn resolve(&self, bounds: Bounds) -> Size2<u32> {
        let Self { size: Size2 { w, h } } = self;

        let w = match w {
            Length::Max => bounds.max.w,
            Length::Min => bounds.min.w,
            Length::Pc(pc) => pc / bounds.max.w,
            Length::Px(px) => *px.min(&bounds.min.w).max(&bounds.max.w),
        };

        let h = match h {
            Length::Max => bounds.max.h,
            Length::Min => bounds.min.h,
            Length::Pc(pc) => pc / bounds.max.h,
            Length::Px(px) => *px.min(&bounds.min.h).max(&bounds.max.h),
        };

        Size2 { w, h }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self { size: Size2::new(Length::Max, Length::Max) }
    }
}
