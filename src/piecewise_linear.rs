use std::cmp::{max, min};
use std::ops::{Add, Sub};
use num_traits::Num;
use std::cmp::Ord;
use std::fmt::Debug;
use itertools::{EitherOrBoth, Itertools};

pub trait CustomNum: Num + Copy + Ord + Debug {}

#[derive(Debug)]
pub struct Point<T: CustomNum>(pub T, pub T);

#[derive(Debug)]
pub struct PiecewiseLinear<T: CustomNum> {
    pub domain: (T, T),
    pub first_slope: T,
    pub last_slope: T,
    pub points: Vec<Point<T>>, // TODO: Maybe use a NonEmptyVec here
}


impl<T: CustomNum> PiecewiseLinear<T> {
    pub fn get_rnk(&self, at: T) -> Result<usize, usize> {
        self.points.binary_search_by_key(&at, |&Point(x, _)| x)
    }

    pub fn eval(&self, at: T) -> T {
        self.eval_with_rank(self.get_rnk(at), at)
    }

    pub fn eval_with_rank(&self, rnk: Result<usize, usize>, at: T) -> T {
        match rnk {
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


fn sum_op<T: CustomNum>(lhs: &PiecewiseLinear<T>, rhs: &PiecewiseLinear<T>, op: fn(T, T) -> T) -> PiecewiseLinear<T> {
    let new_domain = (
        max(lhs.domain.0, rhs.domain.0),
        min(lhs.domain.1, rhs.domain.1)
    );

    let l_domain_changed = new_domain.0 != lhs.domain.0 || new_domain.0 != rhs.domain.0;
    let r_domain_changed = new_domain.1 != lhs.domain.1 || new_domain.1 != rhs.domain.1;

    let mut self_rng = (0, lhs.points.len());
    let mut rhs_rng = (0, rhs.points.len());

    let first_point: Option<Point<T>> = if !l_domain_changed {
        None
    } else {
        let at = new_domain.0;
        let self_rnk = lhs.get_rnk(at);
        let rhs_rnk = rhs.get_rnk(at);

        self_rng.0 = match self_rnk {
            Ok(i) => i,
            Err(i) => i
        };
        rhs_rng.0 = match rhs_rnk {
            Ok(i) => i,
            Err(i) => i
        };
        if self_rnk.is_ok() || rhs_rnk.is_ok() {
            None
        } else {
            Some(Point(new_domain.0, op(lhs.eval_with_rank(self_rnk, at), rhs.eval_with_rank(rhs_rnk, at))))
        }
    };

    let last_point: Option<Point<T>> = if !r_domain_changed {
        None
    } else {
        let at = new_domain.1;
        let self_rnk = lhs.get_rnk(at);
        let rhs_rnk = rhs.get_rnk(at);

        self_rng.1 = match self_rnk {
            Ok(i) => i + 1,
            Err(i) => i
        };
        rhs_rng.1 = match rhs_rnk {
            Ok(i) => i + 1,
            Err(i) => i
        };
        if self_rnk.is_ok() || rhs_rnk.is_ok() {
            None
        } else {
            Some(Point(new_domain.1, op(lhs.eval_with_rank(self_rnk, at), rhs.eval_with_rank(rhs_rnk, at))))
        }
    };

    let capacity = lhs.points.len() + rhs.points.len() + 2;
    let mut new_points: Vec<Point<T>> = Vec::with_capacity(capacity);
    if let Some(value) = first_point { new_points.push(value); }

    let new_iter = lhs.points[self_rng.0..self_rng.1].iter().merge_join_by(
        rhs.points[rhs_rng.0..rhs_rng.1].iter(),
        |x, y| x.0.cmp(&y.0),
    );

    let mut cur_i = self_rng.0;
    let mut cur_j = rhs_rng.0;
    for p in new_iter {
        match p {
            EitherOrBoth::Left(p) => {
                cur_i += 1;
                let val = op(p.1, rhs.eval_with_rank(Err(cur_j), p.0));
                new_points.push(Point(p.0, val));
            }
            EitherOrBoth::Right(q) => {
                cur_j += 1;
                let val = op(lhs.eval_with_rank(Err(cur_i), q.0), q.1);
                new_points.push(Point(q.0, val));
            }
            EitherOrBoth::Both(p, q) => {
                cur_i += 1;
                cur_j += 1;
                let val = op(p.1, q.1);
                new_points.push(Point(q.0, val));
            }
        };
    }

    if let Some(value) = last_point { new_points.push(value); }

    PiecewiseLinear {
        domain: new_domain,
        first_slope: op(lhs.first_slope, rhs.first_slope),
        last_slope: op(lhs.last_slope, rhs.last_slope),
        points: new_points,
    }
}


impl<T: CustomNum> Add<&PiecewiseLinear<T>> for &PiecewiseLinear<T> {
    type Output = PiecewiseLinear<T>;

    fn add(self, rhs: &PiecewiseLinear<T>) -> Self::Output {
        sum_op(self, rhs, |x, y| x + y)
    }
}


impl<T: CustomNum> Sub<&PiecewiseLinear<T>> for &PiecewiseLinear<T> {
    type Output = PiecewiseLinear<T>;

    fn sub(self, rhs: &PiecewiseLinear<T>) -> Self::Output {
        sum_op(self, rhs, |x, y| x - y)
    }
}
