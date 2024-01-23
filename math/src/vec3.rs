use std::borrow::Borrow;
use std::mem::ManuallyDrop;

use super::Vec3;
use crate::{Vec2, Vec4};

union Cast<T> {
    vec3: ManuallyDrop<Vec3<T>>,
    array: ManuallyDrop<[T; 3]>,
}

impl<T> Cast<T> {
    const fn from_vec3(vec3: Vec3<T>) -> Self {
        Self { vec3: ManuallyDrop::new(vec3) }
    }

    const fn to_vec3(self) -> Vec3<T> {
        ManuallyDrop::into_inner(unsafe { self.vec3 })
    }

    const fn from_array(array: [T; 3]) -> Self {
        Self { array: ManuallyDrop::new(array) }
    }

    const fn to_array(self) -> [T; 3] {
        ManuallyDrop::into_inner(unsafe { self.array })
    }
}

impl<T> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn extend(self, w: T) -> Vec4<T> {
        Vec4 { x: self.x, y: self.y, z: self.z, w }
    }

    pub fn truncate(self) -> Vec2<T> {
        Vec2 { x: self.x, y: self.y }
    }

    pub const fn from_array(array: [T; 3]) -> Self {
        Cast::from_array(array).to_vec3()
    }

    pub const fn to_array(self) -> [T; 3] {
        Cast::from_vec3(self).to_array()
    }

    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Vec3<U> {
        Vec3::new(f(self.x), f(self.y), f(self.z))
    }
}

impl<T: Copy> Vec3<T> {
    pub const fn splat(v: T) -> Self {
        let [x, y, z] = [v; 3];

        Self { x, y, z }
    }
}

impl<T> From<[T; 3]> for Vec3<T> {
    fn from(array: [T; 3]) -> Self {
        Cast::from_array(array).to_vec3()
    }
}

impl<T> From<Vec3<T>> for [T; 3] {
    fn from(vec4: Vec3<T>) -> Self {
        vec4.to_array()
    }
}

impl<T> AsRef<[T; 3]> for Vec3<T> {
    fn as_ref(&self) -> &[T; 3] {
        unsafe { &*(self as *const Self).cast() }
    }
}

impl<T> Borrow<[T; 3]> for Vec3<T> {
    fn borrow(&self) -> &[T; 3] {
        self.as_ref()
    }
}
