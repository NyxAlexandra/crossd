//! Type for representing colors.

use std::mem;

use bytemuck::{Pod, Zeroable};
use crossd_math::Vec4;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An RGBA color.
///
/// Values are in the range `[0.0, 1.0]`.
///
/// Is equivalent to `[f32; 4]` or WGSL `vec4<f32>`.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Pod, Zeroable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color {
    /// The red component.
    pub r: f32,
    /// The green component.
    pub g: f32,
    /// The blue component.
    pub b: f32,
    /// The alpha component.
    pub a: f32,
}

impl Color {
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const BLUE: Self = Self::rgb(255, 0, 255);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);

    /// Color from RGB components with 255 alpha.
    ///
    /// ```
    /// # use crossd_graphics::color::Color;
    /// #
    /// let red = Color::rgb(255, 0, 0);
    ///
    /// assert_eq!(red, Color::RED);
    /// assert_eq!(red, Color::rgba(255, 0, 0, 255));
    /// ```
    #[inline]
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Color from RGB components and alpha.
    ///
    /// ```
    /// # use crossd_graphics::color::Color;
    /// let red = Color::new(255, 0, 0, 255);
    ///
    /// assert_eq!(red, Color::RED);
    /// assert_eq!(red, Color::new(0xff, 0x0, 0x0, 0xff));
    /// ```
    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;
        let a = a as f32 / 255.0;

        Self { r, g, b, a }
    }

    #[must_use]
    pub const fn to_rgb(self) -> [u8; 3] {
        let Self { r, g, b, .. } = self;

        [(r * 255.0) as _, (g * 255.0) as _, (b * 255.0) as _]
    }

    #[must_use]
    pub const fn from_rgb(rgb: [u8; 3]) -> Self {
        let [r, g, b] = rgb;

        Self::rgb(r, g, b)
    }

    #[must_use]
    pub const fn to_rgba(self) -> [u8; 4] {
        let Self { r, g, b, a } = self;

        [(r * 255.0) as _, (g * 255.0) as _, (b * 255.0) as _, (a * 255.0) as _]
    }

    #[must_use]
    pub const fn from_rgba(rgba: [u8; 4]) -> Self {
        let [r, g, b, a] = rgba;

        Self::rgba(r, g, b, a)
    }

    /// Create a new color from a `u32`, usually a hex literal (format
    /// `0xAARRGGBB`).
    #[must_use]
    pub const fn from_hex(hex: u32) -> Self {
        Self::from_rgba(unsafe { mem::transmute(hex) })
    }

    /// Convert to a [`Vec4`].
    #[must_use]
    pub const fn to_vec4(self) -> Vec4 {
        unsafe { mem::transmute(self) }
    }

    /// Convert from a [`Vec4`].
    #[must_use]
    pub const fn from_vec4(vec4: Vec4) -> Self {
        unsafe { mem::transmute(vec4) }
    }
}

impl From<[u8; 3]> for Color {
    fn from(rgb: [u8; 3]) -> Self {
        Self::from_rgb(rgb)
    }
}

impl From<Color> for [u8; 3] {
    fn from(color: Color) -> Self {
        color.to_rgb()
    }
}

impl From<[u8; 4]> for Color {
    fn from(rgba: [u8; 4]) -> Self {
        Self::from_rgba(rgba)
    }
}

impl From<Color> for [u8; 4] {
    fn from(color: Color) -> Self {
        color.to_rgba()
    }
}
