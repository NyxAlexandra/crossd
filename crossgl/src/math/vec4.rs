use crate::math::Vec4;

impl<T> Vec4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v, v, v)
    }
}
