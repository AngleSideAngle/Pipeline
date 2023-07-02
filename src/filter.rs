use std::{cell::Cell, ops::Mul};

use dyn_clone::DynClone;

use super::stream::Stream;

pub trait Filter: DynClone {
    fn calculate(&self, input: f64) -> f64;
}

dyn_clone::clone_trait_object!(Filter);

impl<T> Filter for T
where
    T: Fn(f64) -> f64 + Clone,
{
    fn calculate(&self, input: f64) -> f64 {
        self(input)
    }
}

impl Mul<f64> for Stream {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.map(move |x| x * rhs)
    }
}

impl Mul<Stream> for f64 {
    type Output = Stream;

    fn mul(self, rhs: Stream) -> Self::Output {
        Stream::mul(rhs, self)
    }
}

#[derive(Debug, Clone)]
pub struct Derivative {
    last: Cell<f64>,
    period: f64,
}

impl Derivative {
    pub fn new(period: f64) -> Self {
        Self {
            last: Cell::default(),
            period,
        }
    }
}

impl Filter for Derivative {
    fn calculate(&self, input: f64) -> f64 {
        let delta = input - self.last.get();
        self.last.set(input);
        delta / self.period
    }
}

#[derive(Debug, Clone)]
pub struct Integral {
    sum: Cell<f64>,
    period: f64,
}

impl Integral {
    pub fn new(period: f64) -> Self {
        Self {
            sum: Cell::default(),
            period,
        }
    }
}

impl Filter for Integral {
    fn calculate(&self, input: f64) -> f64 {
        self.sum.set(self.sum.get() + input * self.period);
        self.sum.get()
    }
}

impl Stream {
    pub fn differentiate(self, period: Option<f64>) -> Self {
        self.map(Derivative::new(period.unwrap_or(0.02)))
    }

    pub fn integrate(self, period: Option<f64>) -> Self {
        self.map(Integral::new(period.unwrap_or(0.02)))
    }
}
