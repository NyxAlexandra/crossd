use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use bytemuck::Pod;

/// Trait for numbers usable in [`Mat4`](crate::Mat4), [`Vec4`](crate::Vec4),
/// etc.
///
/// Implemented for all primitive types.
pub trait Num:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Pod
{
}

/// Trait for numbers with decimal precision.
///
/// Implemented for `f32` and `f64`.
pub trait Float: Num {
    /// The integer type that can store this float.
    type Int;

    /// Round the decimal, becoming an integer.
    fn round(self) -> Self::Int;

    /// Round the decimal but remain a float.
    fn snap(self) -> Self;
}

/// Traits for [`Num`] types that allow assignment.
///
/// Implemented for all primitive types.
pub trait NumAssign: Num + AddAssign + SubAssign + MulAssign + DivAssign {}

/// Trait for numbers that can have a `1` value.
///
/// Implemented for (signed) integers and floats.
pub trait One: Num {
    /// The 1 value.
    const ONE: Self;
}

/// Trait for numbers that can have a `0` value.
///
/// Implemented for (signed) integers and floats.
pub trait Zero: Num {
    /// The 0 value.
    const ZERO: Self;
}

/// Trait for numbers that can have a `-1` value.
///
/// Implemented for all signed integers and floats.
pub trait NegOne: Num {
    /// The -1 value.
    const NEG_ONE: Self;
}

/// Numbers with a maximum value.
pub trait Max: Num {
    /// The maximum value for this number.
    const MAX: Self;
}

/// Numbers with a minimum value.
pub trait Min: Num {
    /// The minimum value for this number.
    const MIN: Self;
}

// I'm proud of these macros :D

macro_rules! impl_num {
    ($($ty:ident),*) => {
        $(
            impl Num for $ty {}
            impl NumAssign for $ty {}

            impl Max for $ty {
                const MAX: Self = <$ty>::MAX;
            }
            impl Min for $ty {
                const MIN: Self = <$ty>::MIN;
            }
        )*
    };
}

macro_rules! impl_uint {
    ($($ty:ident),*) => {
        $(
            impl_num!($ty);

            impl One for $ty {
                const ONE: Self = 1;
            }
            impl Zero for $ty {
                const ZERO: Self = 0;
            }
        )*
    };
}

macro_rules! impl_int {
    ($($ty:ident),*) => {
        $(
            impl_uint!($ty);

            impl NegOne for $ty {
                const NEG_ONE: Self = -1;
            }
        )*
    };
}

macro_rules! impl_float {
    ($($ty:ident),*) => {
        $(
            impl_num!($ty);

            impl One for $ty {
                const ONE: Self = 1.0;
            }
            impl Zero for $ty {
                const ZERO: Self = 0.0;
            }
            impl NegOne for $ty {
                const NEG_ONE: Self = -1.0;
            }
        )*
    };
}

impl_int![i8, i16, i32, i64, i128, isize];
impl_uint![u8, u16, u32, u64, u128, usize];
impl_float![f32, f64];

impl Float for f32 {
    type Int = i32;

    fn round(self) -> Self::Int {
        self.snap() as _
    }

    fn snap(self) -> Self {
        <f32>::round(self)
    }
}

impl Float for f64 {
    type Int = i64;

    fn round(self) -> Self::Int {
        <f64>::round(self) as _
    }

    fn snap(self) -> Self {
        todo!()
    }
}
