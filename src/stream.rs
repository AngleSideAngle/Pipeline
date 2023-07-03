use std::ops::{Add, Div, Mul, Sub};

use crate::{filter::Filter, junction::Junction, supplier::Supplier};

#[derive(Clone)]
pub enum Stream {
    Leaf(Box<dyn Supplier>),
    Composite(Box<Self>, Box<dyn Filter>),
    Aggregate(Box<Self>, Box<Self>, Box<dyn Junction>),
}

impl Stream {
    pub fn new(supplier: impl Supplier + 'static) -> Self {
        Self::Leaf(Box::new(supplier))
    }

    pub fn map(self, op: impl Filter + 'static) -> Self {
        Self::Composite(Box::new(self), Box::new(op))
    }

    pub fn combine(self, other: Stream, op: impl Junction + 'static) -> Self {
        Self::Aggregate(Box::new(self), Box::new(other), Box::new(op))
    }

    pub fn get(&self) -> f64 {
        match self {
            Self::Leaf(supplier) => supplier.get(),
            Self::Composite(f, filter) => filter.calculate(f.get()),
            Self::Aggregate(f, g, junction) => junction.calculate(f.get(), g.get()),
        }
    }
}

impl Add for Stream {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.combine(rhs, |a, b| a + b)
    }
}

impl Sub for Stream {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.combine(rhs, |a, b| a - b)
    }
}

impl Mul for Stream {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.combine(rhs, |a, b| a * b)
    }
}

impl Div for Stream {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.combine(rhs, |a, b| a / b)
    }
}
