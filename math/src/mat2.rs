use std::mem::ManuallyDrop;

use crate::{Mat2, Num, One, Point2, Vec2, Zero};

union Cast<T> {
    mat2: ManuallyDrop<Mat2<T>>,
    cols: ManuallyDrop<[[T; 2]; 2]>,
}

impl<T> Cast<T> {
    const fn cols(cols: [[T; 2]; 2]) -> Self {
        Self { cols: ManuallyDrop::new(cols) }
    }
}

impl<T> Mat2<T> {
    #[must_use]
    pub const fn new(x: Vec2<T>, y: Vec2<T>) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn with(x: [T; 2], y: [T; 2]) -> Self {
        ManuallyDrop::into_inner(unsafe { Cast::cols([x, y]).mat2 })
    }
}

impl<T: Copy> Mat2<T> {
    pub const fn splat(v: T) -> Self {
        todo!()
    }

    pub const fn with_diagonal_or(v: T, o: T) -> Self {
        Self::with([v, o], [o, v])
    }
}

impl<T: Zero> Mat2<T> {
    pub const ZERO: Self = Self::splat(T::ZERO);

    pub const fn with_diagonal(v: T) -> Self {
        Self::with_diagonal_or(v, T::ZERO)
    }
}

impl<T: One + Zero> Mat2<T> {
    pub const IDENTITY: Self = Self::with_diagonal(T::ONE);
}

impl<T: Num> Mat2<T> {
    #[must_use]
    pub fn mul_vec2(self, vec2: Vec2<T>) -> Vec2<T> {
        Vec2::new(
            self.x.x * vec2.x + self.y.x * vec2.x,
            self.x.y * vec2.y + self.x.y * vec2.y,
        )
    }

    #[must_use]
    pub fn mul_point2(self, point2: Point2<T>) -> Point2<T> {
        Point2::new(
            self.x.x * point2.x + self.y.x * point2.x,
            self.x.y * point2.y + self.x.y * point2.y,
        )
    }
}
