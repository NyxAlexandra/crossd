#![allow(unused)]

#[macro_export]
macro_rules! assert_compiles {
    ($ty:ty) => {
        const _: Option<&$ty> = None;
    };
}
