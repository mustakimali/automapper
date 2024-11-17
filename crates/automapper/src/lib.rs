pub use ::automapper_proc::map as ___map;

pub trait AutoMapsTo {
    type Destination;
    fn map_to(self) -> Self::Destination;
}

#[macro_export]
macro_rules! map {
    ($s: path, $d: path) => {
        impl ::automapper::AutoMapsTo for $s {
            type Destination = $d;
            fn map_to(self) -> Self::Destination {
                ::automapper::___map! {
                    fn __map($s, $d);
                }

                __map(self)
            }
        }
    };
}
