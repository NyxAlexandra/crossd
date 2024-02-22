use crate::Vec3;

impl<T> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v, v)
    }

    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Vec3<U> {
        let Self { x, y, z } = self;

        let x = f(x);
        let y = f(y);
        let z = f(z);

        Vec3 { x, y, z }
    }

    pub fn cast<U>(self) -> Vec3<U>
    where
        T: Into<U>,
    {
        self.map(Into::into)
    }
}
