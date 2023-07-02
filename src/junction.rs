use dyn_clone::DynClone;

pub trait Junction: DynClone {
    fn calculate(&self, a: f64, b: f64) -> f64;
}

dyn_clone::clone_trait_object!(Junction);

impl<T> Junction for T
where
    T: Fn(f64, f64) -> f64 + Clone,
{
    fn calculate(&self, a: f64, b: f64) -> f64 {
        self(a, b)
    }
}
