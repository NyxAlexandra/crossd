use std::mem::ManuallyDrop;

use bytemuck::{Pod, Zeroable};

use super::Vec2;

/// A union for casting between different representations of a [`Vec2`].
#[repr(C)]
union Cast<T> {
    vec2: ManuallyDrop<Vec2<T>>,

    array: ManuallyDrop<[T; 2]>,
    tuple: ManuallyDrop<(T, T)>,
}

impl<T> Vec2<T> {
    #[must_use]
    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; 2]) -> Self {
        ManuallyDrop::into_inner(unsafe { Cast { array: ManuallyDrop::new(array) }.vec2 })
    }

    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 2] {
        ManuallyDrop::into_inner(unsafe { Cast { vec2: ManuallyDrop::new(self) }.array })
    }

    #[inline]
    #[must_use]
    pub const fn from_tuple(tuple: (T, T)) -> Self {
        ManuallyDrop::into_inner(unsafe { Cast { tuple: ManuallyDrop::new(tuple) }.vec2 })
    }

    #[inline]
    #[must_use]
    pub const fn to_tuple(self) -> (T, T) {
        ManuallyDrop::into_inner(unsafe { Cast { vec2: ManuallyDrop::new(self) }.tuple })
    }

    #[inline]
    #[must_use]
    pub fn map<U>(self, f: impl FnMut(T) -> U) -> Vec2<U> {
        let mut f = f;

        Vec2::new(f(self.x), f(self.y))
    }
}

impl<T: Copy> Vec2<T> {
    #[inline]
    #[must_use]
    pub const fn splat(v: T) -> Self {
        Self { x: v, y: v }
    }
}

unsafe impl<T: Pod> Pod for Vec2<T> {}
unsafe impl<T: Zeroable> Zeroable for Vec2<T> {}
