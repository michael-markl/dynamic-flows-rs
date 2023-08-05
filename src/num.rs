use core::fmt::Debug;
use num_traits::{Num as num_traits_Num, NumAssignOps, Signed};
use std::{fmt::Display, hash::Hash, ops::Neg};

pub trait Num:
    num_traits_Num
    + Neg<Output = Self>
    + Signed
    + Ord
    + Copy
    + Debug
    + Display
    + Hash
    + NumAssignOps
    + std::iter::Sum
{
    const EXACT_ARITHMETIC: bool;
    const ZERO: Self;
    const ONE: Self;
    const TOL: Self;
    const INFINITY: Self;
    fn to_f64(self) -> f64;
}

pub trait Sum: for<'a> Iterator {
    fn sum_iter<'a, T: Num + 'a>(self) -> T
    where
        Self: Iterator<Item = &'a T> + Sized,
    {
        self.fold(T::ZERO, |mut acc, x| {
            acc += *x;
            return acc;
        })
    }
}

impl<'a, T: Num + 'a, I: Iterator<Item = &'a T>> Sum for I {}
