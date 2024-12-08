#![doc = include_str!("../README.md")]

pub mod macros {
    /// Re-exported from [automapper_proc::impl_map_fn]
    ///
    pub use ::automapper_proc::impl_map_fn;
}

pub trait AutoMapsTo<Dest> {
    fn map_to(self) -> Dest;
}

/// This trait is the opposite of `AutoMapsTo`.
/// This is automatically implemented for any type that implements `AutoMapsTo`.
///
/// For any type where you can do
/// ```norun
/// let dest: D = source.map_to();
/// ```
///
/// You can also do
/// ```norun
/// let dest = DestType::map_from(source);
/// ```
pub trait AutoMapsFrom<T> {
    fn map_from(src: T) -> Self;
}

#[macro_export]
macro_rules! map {
    ($s: path => $d: path) => {
        impl ::automapper::AutoMapsTo<$d> for $s {
            fn map_to(self) -> $d {
                ::automapper::macros::impl_map_fn! {
                    fn __map($s) -> $d;
                }

                __map(self)
            }
        }
    };
}

impl<D, S> AutoMapsFrom<S> for D
where
    S: AutoMapsTo<D>,
{
    fn map_from(src: S) -> Self {
        S::map_to(src)
    }
}
