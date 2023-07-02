use std::ops::{Add, Div, Mul, Sub};

use super::filter::Filter;

type Supplier = fn() -> f64;

#[derive(Clone)]
pub enum Stream {
    Supplier(Supplier),
    Aggregate(Box<Stream>, Box<Stream>, fn(f64, f64) -> f64),
    Composite(Box<Stream>, Box<dyn Filter>),
}

impl Stream {
    pub fn new(supplier: Supplier) -> Self {
        Self::Supplier(supplier)
    }

    pub fn get(&self) -> f64 {
        match self {
            Stream::Supplier(f) => f(),
            Stream::Aggregate(f, g, op) => op(f.to_owned().get(), g.to_owned().get()),
            Stream::Composite(f, filter) => filter.calculate(f.to_owned().get()),
        }
    }

    pub fn map(self, op: impl Filter + 'static) -> Self {
        Stream::Composite(Box::new(self), Box::new(op))
    }
}

impl Add for Stream {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Stream::Aggregate(Box::new(self), Box::new(rhs), |x, y| x + y)
    }
}

impl Sub for Stream {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Stream::Aggregate(Box::new(self), Box::new(rhs), |x, y| x - y)
    }
}

impl Mul for Stream {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Stream::Aggregate(Box::new(self), Box::new(rhs), |x, y| x * y)
    }
}

impl Div for Stream {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Stream::Aggregate(Box::new(self), Box::new(rhs), |x, y| x / y)
    }
}
