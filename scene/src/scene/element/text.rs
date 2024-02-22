use std::borrow::Cow;
use std::ops::Range;

use crate::Size2;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Text {
    pub content: Cow<'static, str>,
    pub bounds: Size2<f32>,
    pub selection: Option<Selection>,

    pub font: Font,
    pub line_height: f32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Font {
    pub size: f32,
    pub family: FontFamily,
    pub weight: FontWeight,
    pub stretch: FontStretch,
    pub style: FontStyle,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub enum FontFamily {
    Name(Cow<'static, str>),
    Serif,
    #[default]
    SansSerif,
    Monospace,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    #[default]
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Selection {
    pub anchor: usize,
    pub active: usize,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            size: 14.0,
            family: FontFamily::default(),
            weight: FontWeight::default(),
            stretch: FontStretch::default(),
            style: FontStyle::default(),
        }
    }
}

impl Selection {
    pub const fn new(range: Range<usize>) -> Self {
        let Range { start: anchor, end: active } = range;

        Self { anchor, active }
    }

    pub const fn caret(index: usize) -> Self {
        Self { anchor: index, active: index }
    }

    pub fn copy(self, s: &str) -> &str {
        if self.anchor < self.active {
            &s[self.anchor..self.active]
        } else {
            &s[self.active..self.anchor]
        }
    }
}
