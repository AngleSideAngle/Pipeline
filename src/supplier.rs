use dyn_clone::DynClone;

use crate::stream::Stream;

pub trait Supplier: DynClone {
    fn get(&self) -> f64;

    fn stream(self) -> Stream
    where
        Self: Sized + 'static,
    {
        Stream::new(self)
    }
}

dyn_clone::clone_trait_object!(Supplier);

impl<T> Supplier for T
where
    T: Fn() -> f64 + Clone,
{
    fn get(&self) -> f64 {
        self()
    }
}
