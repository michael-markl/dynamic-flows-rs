use core::fmt::Debug;
use num_traits::Num as num_traits_Num;
use std::{fmt::Display, ops::Neg};

pub trait Num: num_traits_Num + Neg<Output = Self> + Copy + Ord + Debug + Display {
    const EXACT_ARITHMETIC: bool;
    const ZERO: Self;
    const ONE: Self;
    const TOL: Self;
    const INFINITY: Self;
    fn to_f64(self) -> f64;
}
