use std::mem;

use bytemuck::{Pod, Zeroable};

use crate::math::Vec4;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Pod, Zeroable)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn gray(v: u8) -> Self {
        Self::rgb(v, v, v)
    }
}

impl From<u32> for Color {
    fn from(num: u32) -> Self {
        unsafe { mem::transmute(num) }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        unsafe { mem::transmute(color) }
    }
}

impl From<Vec4<u8>> for Color {
    fn from(vec: Vec4<u8>) -> Self {
        unsafe { mem::transmute(vec) }
    }
}

impl From<Color> for Vec4<u8> {
    fn from(color: Color) -> Self {
        unsafe { mem::transmute(color) }
    }
}

impl From<Color> for Vec4<f32> {
    fn from(Color { r, g, b, a }: Color) -> Self {
        Vec4::new(
            (r as f32) / 255.0,
            (g as f32) / 255.0,
            (b as f32) / 255.0,
            (a as f32) / 255.0,
        )
    }
}
