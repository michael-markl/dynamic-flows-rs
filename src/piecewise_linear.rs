use itertools::{EitherOrBoth, Itertools};
use std::cmp::{max, min};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Neg, Sub};

use crate::num::Num;
use crate::point::Point;

#[derive(Debug)]
pub struct PiecewiseLinear<T: Num> {
    pub domain: (T, T),
    first_slope: T,
    last_slope: T,
    pub points: Vec<Point<T>>, // TODO: Maybe use a NonEmptyVec here
}

impl<T: Num> PiecewiseLinear<T> {
    pub fn new(
        domain: (impl Into<T>, impl Into<T>),
        first_slope: impl Into<T>,
        last_slope: impl Into<T>,
        points: Vec<Point<T>>,
    ) -> Self {
        let domain: (T, T) = (domain.0.into(), domain.1.into());
        let first_slope: T = first_slope.into();
        let last_slope: T = last_slope.into();
        let domain: (T, T) = (domain.0.into(), domain.1.into());
        debug_assert!(domain.0 <= domain.1, "The domain is not well defined.");
        debug_assert!(points.len() >= 1, "There must be at least one point.");
        debug_assert!(
            points[0].0 >= domain.0,
            "The first point is not in the domain."
        );
        debug_assert!(
            points[points.len() - 1].0 <= domain.1,
            "The last point is not in the domain."
        );
        debug_assert!(
            points.windows(2).all(|w| w[0].0 < w[1].0),
            "The points are not sorted by x-coordinate."
        );

        Self {
            domain,
            first_slope,
            last_slope,
            points,
        }
    }

    pub fn get_rnk(&self, at: T) -> Result<usize, usize> {
        self.points.binary_search_by_key(&at, |&Point(x, _)| x)
    }

    pub fn eval(&self, at: impl Into<T>) -> T {
        let at = at.into();
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
                    let left = &self.points[rnk - 1];
                    let right = &self.points[rnk];
                    left.1 + (at - left.0) * (right.1 - left.1) / (right.0 - left.0)
                }
            }
        }
    }

    /// Returns the gradient between `points[i-1].0` (or `domain.0` if `i == 0`) and `times[i]`
    /// (or `domain.1` if `i == len(times)`)
    pub fn gradient(&self, i: usize) -> T {
        // TODO: The following code has not been tested!

        debug_assert!(i <= self.points.len(), "i is not in the expected range.");
        if i == 0 {
            self.first_slope
        } else if i == self.points.len() {
            self.last_slope
        } else {
            let p = &self.points[i - 1];
            let q = &self.points[i];
            (q.1 - p.1) / (q.0 - p.0)
        }
    }

    /// Returns the composition h(x):= self(rhs(x))
    pub fn compose(&self, rhs: &PiecewiseLinear<T>) -> PiecewiseLinear<T> {
        // TODO: The following code has not been tested!
        let g = self;
        let f = rhs;

        debug_assert!(
            f.is_monotone(),
            "Composition g ⚬ f is only implemented for f monotone increasing."
        );
        debug_assert!(
            {
                let f_img = f.image();
                g.domain.0 <= f_img.0 + T::TOL && g.domain.1 >= f_img.1 - T::TOL
            },
            "The domains do not match for composition."
        );

        let mut points: Vec<Point<T>> = Vec::new(); // todo: add some heuristic capacity
        let f_img = f.image();
        let g_rnk = match g.get_rnk(f_img.0) {
            Ok(i) => i,
            Err(i) => i,
        };
        // g.points[g_rnk - 1].0 < f_img.0 <= g.points[g_rnk].0
        let first_slope = g.gradient(g_rnk) * f.first_slope; // By chain rule

        let mut i_g = max(1, g_rnk); // Start of interval

        debug_assert!(i_g == g.points.len() - 1 || f.domain.0 <= g.points[i_g + 1].0);

        for i_f in 0..f.points.len() {
            // Interval (f.points[i_f - 1], f.points[i_f])
            while i_g < g.points.len() && g.points[i_g - 1].0 <= f.points[i_f].1 {
                let next_time = max(f_img.0, g.points[i_g - 1].0);
                if f.gradient(i_f) != T::ZERO {
                    let inv = f.inverse(next_time, i_f);
                    if points.last().map_or(true, |x| inv > x.0 + T::TOL) {
                        let p = Point(inv, g.eval(next_time)); // todo: use rnk for g
                        points.push(p);
                    }
                }
                i_g += 1;
            }
            if points
                .last()
                .map_or(true, |x| f.points[i_f].0 > x.0 + T::TOL)
            {
                let p = Point(f.points[i_f].0, g.eval(f.points[i_f].1)); // todo: use rnk for g
                points.push(p);
            }
        }

        while i_g <= g.points.len() && g.points[i_g - 1].0 <= f_img.1 {
            let next_time = max(f_img.0, g.points[i_g - 1].0);
            if f.gradient(f.points.len()) != T::ZERO {
                let inv = f.inverse(next_time, f.points.len()); // todo: check usages of inverse
                if points.last().map_or(true, |x| inv > x.0 + T::TOL) {
                    let p = Point(inv, g.eval(next_time)); // todo: use rnk for g
                    points.push(p);
                }
            }
            i_g += 1;
        }

        let last_slope = g.gradient(i_g) * f.last_slope;
        return PiecewiseLinear {
            domain: f.domain,
            first_slope,
            last_slope,
            points,
        };
    }

    fn is_monotone(&self) -> bool {
        return self.first_slope >= T::ZERO
            && self.last_slope >= T::ZERO
            && self.points.windows(2).all(|w| w[0].1 <= w[1].1);
    }
    fn image(&self) -> (T, T) {
        debug_assert!(
            self.is_monotone(),
            "Only implemented for monotone functions."
        );
        // TODO: The performance could be improved by guessing the rank.
        return (self.eval(self.domain.0), self.eval(self.domain.1));
    }

    fn inverse(&self, p0: T, p1: usize) -> T {
        todo!("Not yet implemented!")
    }
}

fn sum_op<T: Num, F: Fn(T, T) -> T>(
    lhs: &PiecewiseLinear<T>,
    rhs: &PiecewiseLinear<T>,
    op: F,
) -> PiecewiseLinear<T> {
    let new_domain = (
        max(lhs.domain.0, rhs.domain.0),
        min(lhs.domain.1, rhs.domain.1),
    );

    let l_domain_changed = new_domain.0 != lhs.domain.0 || new_domain.0 != rhs.domain.0;
    let r_domain_changed = new_domain.1 != lhs.domain.1 || new_domain.1 != rhs.domain.1;

    let mut lhs_rng = (0, lhs.points.len());
    let mut rhs_rng = (0, rhs.points.len());

    let first_point: Option<Point<T>> = if !l_domain_changed {
        None
    } else {
        let at = new_domain.0;
        let lhs_rnk = lhs.get_rnk(at);
        let rhs_rnk = rhs.get_rnk(at);

        lhs_rng.0 = match lhs_rnk {
            Ok(i) => i,
            Err(i) => i,
        };
        rhs_rng.0 = match rhs_rnk {
            Ok(i) => i,
            Err(i) => i,
        };
        if lhs_rnk.is_ok() || rhs_rnk.is_ok() {
            None
        } else {
            Some(Point(
                new_domain.0,
                op(
                    lhs.eval_with_rank(lhs_rnk, at),
                    rhs.eval_with_rank(rhs_rnk, at),
                ),
            ))
        }
    };

    let last_point: Option<Point<T>> = if !r_domain_changed {
        None
    } else {
        let at = new_domain.1;
        let lhs_rnk = lhs.get_rnk(at);
        let rhs_rnk = rhs.get_rnk(at);

        lhs_rng.1 = match lhs_rnk {
            Ok(i) => i + 1,
            Err(i) => i,
        };
        rhs_rng.1 = match rhs_rnk {
            Ok(i) => i + 1,
            Err(i) => i,
        };
        if lhs_rnk.is_ok() || rhs_rnk.is_ok() {
            None
        } else {
            Some(Point(
                new_domain.1,
                op(
                    lhs.eval_with_rank(lhs_rnk, at),
                    rhs.eval_with_rank(rhs_rnk, at),
                ),
            ))
        }
    };

    // This is a worst-case capacity.
    // For better memory-usage, we could first find out how many elements we exactly need before allocating.
    // That potentially comes with a performance drawback.
    let capacity = lhs.points.len() + rhs.points.len() + 2;
    let mut new_points: Vec<Point<T>> = Vec::with_capacity(capacity);

    if let Some(value) = first_point {
        new_points.push(value);
    }

    let new_iter = lhs.points[lhs_rng.0..lhs_rng.1]
        .iter()
        .merge_join_by(rhs.points[rhs_rng.0..rhs_rng.1].iter(), |x, y| {
            x.0.cmp(&y.0)
        });

    let mut cur_i = lhs_rng.0;
    let mut cur_j = rhs_rng.0;

    // Returns true, if `t` lies in the future by an offset of T::TOL.
    let time_in_tolerance = |t: T, list: &Vec<Point<T>>| -> bool {
        match list.last() {
            None => true,
            Some(p) => p.0 < (t - T::TOL),
        }
    };

    for p in new_iter {
        match p {
            EitherOrBoth::Left(p) => {
                cur_i += 1;
                let val = op(p.1, rhs.eval_with_rank(Err(cur_j), p.0));
                if T::EXACT_ARITHMETIC || time_in_tolerance(p.0, &new_points) {
                    new_points.push(Point(p.0, val));
                }
            }
            EitherOrBoth::Right(q) => {
                cur_j += 1;
                let val = op(lhs.eval_with_rank(Err(cur_i), q.0), q.1);
                if T::EXACT_ARITHMETIC || time_in_tolerance(q.0, &new_points) {
                    new_points.push(Point(q.0, val));
                }
            }
            EitherOrBoth::Both(p, q) => {
                cur_i += 1;
                cur_j += 1;
                let val = op(p.1, q.1);
                if T::EXACT_ARITHMETIC || time_in_tolerance(q.0, &new_points) {
                    new_points.push(Point(q.0, val));
                }
            }
        };
    }

    if let Some(value) = last_point {
        new_points.push(value);
    }

    PiecewiseLinear {
        domain: new_domain,
        first_slope: op(lhs.first_slope, rhs.first_slope),
        last_slope: op(lhs.last_slope, rhs.last_slope),
        points: new_points,
    }
}

impl<T: Num> Add<&PiecewiseLinear<T>> for &PiecewiseLinear<T> {
    type Output = PiecewiseLinear<T>;

    #[inline]
    fn add(self, rhs: &PiecewiseLinear<T>) -> Self::Output {
        sum_op(self, rhs, |a, b| a + b)
    }
}

impl<T: Num> Sub<&PiecewiseLinear<T>> for &PiecewiseLinear<T> {
    type Output = PiecewiseLinear<T>;

    #[inline]
    fn sub(self, rhs: &PiecewiseLinear<T>) -> Self::Output {
        sum_op(self, rhs, |a, b| a - b)
    }
}

impl<T: Num> Neg for &PiecewiseLinear<T> {
    type Output = PiecewiseLinear<T>;

    fn neg(self) -> Self::Output {
        return PiecewiseLinear::new(
            self.domain,
            -self.first_slope,
            -self.last_slope,
            self.points.iter().map(|p| Point(p.0, -p.1)).collect_vec(),
        );
    }
}

impl<T: Num> Display for PiecewiseLinear<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PiecewiseLinear {{ ")?;
        write!(f, "domain: ({:},{:}), ", self.domain.0, self.domain.1)?;
        write!(f, "first_slope: {:}, ", self.first_slope)?;
        write!(f, "last_slope: {:}, ", self.last_slope)?;
        write!(f, "points: [ ")?;
        for p in &self.points {
            write!(f, "{:}, ", p)?;
        }
        write!(f, "] ")?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{float::F64, piecewise_linear::PiecewiseLinear, point::Point, points};

    #[test]
    fn it_adds_two_piecewise_linear_functions() {
        let f: PiecewiseLinear<F64> =
            PiecewiseLinear::new((0.0, 1.0), 1.0, 1.0, points![(0.0, 0.0), (1.0, 1.0)]);
        let g: PiecewiseLinear<F64> =
            PiecewiseLinear::new((0.0, 1.0), 1.0, 1.0, points![(0.0, 0.0), (1.0, 1.0)]);
        let h = &f + &g;
        assert_eq!(h.eval(0.0), 0.0);
        assert_eq!(h.eval(0.5), 1.0);
        assert_eq!(h.eval(1.0), 2.0);
        assert_eq!(h.points, points![(0.0, 0.0), (1.0, 2.0)]);
    }
}
