use std::cmp::{max, min, Ordering};
use std::ops::Add;
use num_traits::Num;
use std::cmp::Ord;

pub struct Point<T: Num>(pub T, pub T);

pub struct PiecewiseLinear<T: Num> {
    pub domain: (T, T),
    pub first_slope: T,
    pub last_slope: T,
    pub points: Vec<Point<T>>,
}

impl<T: Num + Copy + Ord> PiecewiseLinear<T> {
    pub(crate) fn eval(&self, at: T) -> T {
        match self.points.binary_search_by_key(&at, |&Point(x, y)| x) {
            Ok(rnk) => self.points[rnk].1.clone(),
            Err(rnk) => {
                if rnk == self.points.len() {
                    let last = &self.points[rnk - 1];
                    last.1 + (at - last.0) * self.last_slope
                } else if rnk == 0 {
                    let first = &self.points[rnk];
                    first.1 + (at - first.0) * self.first_slope
                } else {
                    let left = &self.points[rnk];
                    let right = &self.points[rnk + 1];
                    left.1 + (at - left.0) * (right.1 - left.1) / (right.0 - left.0)
                }
            }
        }
    }
}

