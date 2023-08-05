use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use num_traits::{Num as NumTraitsNum, One, Signed, Zero};
use ordered_float::OrderedFloat;

use crate::num::Num;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct F64(OrderedFloat<f64>);

impl Into<F64> for OrderedFloat<f64> {
    #[inline]
    fn into(self) -> F64 {
        F64(self)
    }
}

impl Into<F64> for f64 {
    #[inline]
    fn into(self) -> F64 {
        OrderedFloat(self).into()
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
        self.0.neg().into()
    }
}

impl Hash for F64 {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl std::iter::Sum for F64 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.map(|x| x.0 .0).sum::<f64>().into()
    }
}

impl Signed for F64 {
    #[inline]
    fn abs(&self) -> Self {
        return self.0.abs().into();
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        return self.0.abs_sub(&other.0).into();
    }

    #[inline]
    fn signum(&self) -> Self {
        return self.0.signum().into();
    }

    #[inline]
    fn is_positive(&self) -> bool {
        return self.0.is_positive();
    }

    #[inline]
    fn is_negative(&self) -> bool {
        return self.0.is_negative();
    }
}

impl AddAssign for F64 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0);
    }
}

impl SubAssign for F64 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(rhs.0);
    }
}

impl MulAssign for F64 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0.mul_assign(rhs.0);
    }
}

impl RemAssign for F64 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.0.rem_assign(rhs.0);
    }
}

impl DivAssign for F64 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0.div_assign(rhs.0);
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
