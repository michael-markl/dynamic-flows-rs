use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use num_traits::{Num as NumTraitsNum, One, Zero};
use ordered_float::OrderedFloat;

use crate::num::Num;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct F64(OrderedFloat<f64>);

impl From<f64> for F64 {
    #[inline]
    fn from(val: f64) -> Self {
        Self(OrderedFloat(val))
    }
}

impl Rem for F64 {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0.rem(rhs.0))
    }
}

impl Div for F64 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.div(rhs.0))
    }
}

impl Sub for F64 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}

impl Mul for F64 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}

impl Add for F64 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl Zero for F64 {
    #[inline]
    fn zero() -> Self {
        Self(OrderedFloat::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for F64 {
    #[inline]
    fn one() -> Self {
        return Self(OrderedFloat::one());
    }
}

impl PartialEq<F64> for F64 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialEq<f64> for F64 {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd for F64 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }
}

impl Eq for F64 {}

impl Ord for F64 {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl NumTraitsNum for F64 {
    type FromStrRadixErr = <OrderedFloat<f64> as NumTraitsNum>::FromStrRadixErr;

    #[inline]
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        OrderedFloat::from_str_radix(str, radix).map(Self)
    }
}

impl Display for F64 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0 .0, f)
    }
}

impl Neg for F64 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl Num for F64 {
    const EXACT_ARITHMETIC: bool = false;
    const TOL: Self = F64(OrderedFloat(1e-9));
    const ZERO: Self = F64(OrderedFloat(0.));
    const ONE: Self = F64(OrderedFloat(1.));
    const INFINITY: Self = F64(OrderedFloat(f64::INFINITY));

    #[inline]
    fn to_f64(self) -> f64 {
        self.0 .0
    }
}
