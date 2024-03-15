/// Cast `$val` between type `$a` and `$b`.
///
/// ## Safety
///
/// - `$a` and `$b` must be the same size.
/// - `$a` must not have any padding bytes.
/// - `$val` must be a valid instance of `$b`.
#[macro_export]
macro_rules! transmute {
    (for<$($t:ident),*> $a:ty, $b:ty, $val:expr) => {
        {
            #[repr(C)]
            union Cast<$($t),*> {
                a: ::std::mem::ManuallyDrop<$a>,
                b: ::std::mem::ManuallyDrop<$b>,
            }

            ::std::mem::ManuallyDrop::into_inner(
                Cast { a: ::std::mem::ManuallyDrop::new($val) }.b
            )
        }
    };
    ($a:ty, $b:ty, $val:expr) => {
        ::std::mem::transmute::<$a, $b>($val)
    };
}

pub extern crate wgpu;

pub mod color;
pub mod math;
pub mod num;
pub mod scene;
