//! Type for representing colors.

use std::mem;

use bytemuck::{Pod, Zeroable};

/// An RGBA color.
///
/// Is equivalent to `[u8; 4]` or WGSL `vec4<u8>`.
#[repr(C)]
#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Pod, Zeroable,
)]
pub struct Color {
    /// The red component.
    pub r: u8,
    /// The green component.
    pub g: u8,
    /// The blue component.
    pub b: u8,
    /// The alpha component.
    pub a: u8,
}

impl Color {
    /// Color black.
    ///
    /// Fully opaque.
    pub const BLACK: Self = Self::new_opaque(0, 0, 0);
    /// Blue 255.
    ///
    /// Fully opaque.
    pub const BLUE: Self = Self::new_opaque(0, 0, 255);
    /// Green 255.
    ///
    /// Fully opaque.
    pub const GREEN: Self = Self::new_opaque(0, 255, 0);
    /// Red 255.
    ///
    /// Fully opaque.
    pub const RED: Self = Self::new_opaque(255, 0, 0);
    /// Color white.
    ///
    /// Fully opaque.
    pub const WHITE: Self = Self::splat_opaque(255);

    /// Color from RGB components and alpha.
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Color from RGB components with an alpha of 255.
    #[must_use]
    pub const fn new_opaque(r: u8, g: u8, b: u8) -> Self {
        let a = 255;

        Self { r, g, b, a }
    }

    /// A color with each component set to `v`.
    #[must_use]
    pub const fn splat(v: u8) -> Self {
        Self::from_array([v; 4])
    }

    /// A color with RGB set to `v` using alpha 255.
    #[must_use]
    pub const fn splat_opaque(v: u8) -> Self {
        let a = 255;

        Self::new(v, v, v, a)
    }

    /// Convert to an array `[r, g, b, a]`.
    #[must_use]
    pub const fn to_array(self) -> [u8; 4] {
        unsafe { mem::transmute(self) }
    }

    /// Color from array `[r, g, b, a]`.
    #[must_use]
    pub const fn from_array(rgba: [u8; 4]) -> Self {
        unsafe { mem::transmute(rgba) }
    }

    /// Convert to the hexadecimal representation of the color (`0xAARRGGBB`).
    ///
    /// ```
    /// # use crossd_graphics::color::Color;
    /// let color = Color::BLACK;
    /// let hex = color.to_hex();
    ///
    /// assert_eq!(hex, 0xFF000000);
    /// ```
    #[must_use]
    pub const fn to_hex(self) -> u32 {
        // my beloved function
        //
        // this is so cool
        unsafe { mem::transmute(self) }
    }

    /// Color from a hex code of format `0xAARRGGBB`.
    ///
    /// ```
    /// # use crossd_graphics::color::Color;
    /// let hex = 0xffffffff;
    /// let color = Color::from_hex(hex);
    ///
    /// assert_eq!(color, Color::WHITE);
    /// ```
    #[must_use]
    pub const fn from_hex(hex: u32) -> Self {
        unsafe { mem::transmute(hex) }
    }

    /// Convert to [Wgpu's color type](wgpu::Color).
    ///
    /// ```
    /// # use crossd_graphics::color::Color;
    /// #
    /// assert_eq!(Color::WHITE.to_wgpu(), wgpu::Color::WHITE);
    /// ```
    #[must_use]
    pub fn to_wgpu(self) -> wgpu::Color {
        wgpu::Color {
            r: (self.r as f64) / 255.0,
            g: (self.g as f64) / 255.0,
            b: (self.b as f64) / 255.0,
            a: (self.a as f64) / 255.0,
        }
    }

    /// Convert from [Wgpu's color type](wgpu::Color).
    ///
    /// ```
    /// # use crossd_graphics::color::Color;
    /// #
    /// let white = wgpu::Color::WHITE;
    ///
    /// assert_eq!(Color::from_wgpu(white), Color::WHITE);
    /// ```
    #[must_use]
    pub fn from_wgpu(wgpu: wgpu::Color) -> Self {
        Self::new(
            (wgpu.r * 255.0) as _,
            (wgpu.g * 255.0) as _,
            (wgpu.b * 255.0) as _,
            (wgpu.a * 255.0) as _,
        )
    }
}

impl From<[u8; 4]> for Color {
    fn from(rgba: [u8; 4]) -> Self {
        Self::from_array(rgba)
    }
}

impl From<Color> for [u8; 4] {
    fn from(color: Color) -> Self {
        color.to_array()
    }
}

impl AsRef<[u8; 4]> for Color {
    fn as_ref(&self) -> &[u8; 4] {
        unsafe { &*(self as *const Self).cast() }
    }
}

impl AsRef<[u8]> for Color {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self as *const Self as *const [u8; 4] as *const [u8]) }
    }
}
