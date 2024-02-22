use std::mem;

use crate::Vec4;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn gray(v: f32) -> Self {
        Self::rgba(v, v, v, 1.0)
    }
}

impl From<Vec4<f32>> for Color {
    fn from(Vec4 { x, y, z, w }: Vec4<f32>) -> Self {
        Self::rgba(x, y, z, w)
    }
}

impl From<Color> for Vec4<f32> {
    fn from(Color { r, g, b, a }: Color) -> Self {
        Self::new(r, g, b, a)
    }
}

impl From<[f32; 4]> for Color {
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Color> for [f32; 4] {
    fn from(Color { r, g, b, a }: Color) -> Self {
        [r, g, b, a]
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl From<Color> for (f32, f32, f32, f32) {
    fn from(Color { r, g, b, a }: Color) -> Self {
        (r, g, b, a)
    }
}

impl From<[u8; 4]> for Color {
    fn from(rgba: [u8; 4]) -> Self {
        Self::from(rgba.map(|u8| u8 as f32 / 255.0))
    }
}

impl From<Color> for [u8; 4] {
    fn from(Color { r, g, b, a }: Color) -> Self {
        [r, g, b, a].map(|f32| (f32.clamp(0.0, 1.0) * 255.0) as u8)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::from([r, g, b, a])
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    fn from(color: Color) -> Self {
        let [r, g, b, a]: [u8; 4] = color.into();

        (r, g, b, a)
    }
}

impl From<u32> for Color {
    fn from(hex: u32) -> Self {
        let bytes: [u8; 4] = unsafe { mem::transmute(hex) };

        Self::from(bytes)
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        Self::from_be_bytes(color.into())
    }
}
