mod circle;
mod mat3;
mod point2;
mod rect;
mod size2;
mod triangle;
mod vec2;
mod vec3;
mod vec4;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mat3<T> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mat4<T> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size2<T> {
    pub w: T,
    pub h: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect<T> {
    pub point: Point2<T>,
    pub size: Size2<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Circle<T> {
    pub point: Point2<T>,
    pub radius: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Triangle<T>(pub Point2<T>, pub Point2<T>, pub Point2<T>);

macro_rules! impl_bytemuck {
    ($($t:ident)*) => {
        $(
            unsafe impl<T> ::bytemuck::Pod for $t<T>
            where
                T: ::bytemuck::Pod,
            {}
            unsafe impl<T> ::bytemuck::Zeroable for $t<T>
            where
                T: ::bytemuck::Zeroable,
            {}
        )*
    };
}

impl_bytemuck!(Vec2 Vec3 Vec4 Mat3 Mat4 Point2 Point3 Size2 Rect Circle Triangle);