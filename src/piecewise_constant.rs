use num_traits::abs;

use crate::num::Num;
use crate::point::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct PiecewiseConstant<T: Num> {
    domain: [T; 2],
    points: Vec<Point<T>>, // TODO: Maybe use a NonEmptyVec here
}

impl<T: Num> PiecewiseConstant<T> {
    pub fn new(domain: [impl Into<T>; 2], points: Vec<Point<T>>) -> Self {
        let domain = domain.map(|x| x.into());
        debug_assert!(domain[0] <= domain[1], "The domain is not well defined.");
        debug_assert!(!points.is_empty(), "There must be at least one point.");
        debug_assert!(
            points[0].0 >= domain[0],
            "The first point is not in the domain."
        );
        debug_assert!(
            points[points.len() - 1].0 <= domain[1],
            "The last point is not in the domain."
        );
        debug_assert!(
            points.windows(2).all(|w| w[0].0 < w[1].0),
            "The points are not sorted by x-coordinate."
        );

        Self { domain, points }
    }

    pub fn domain(&self) -> [T; 2] {
        self.domain
    }

    pub fn points(&self) -> &[Point<T>] {
        &self.points
    }

    pub fn get_rnk(&self, at: T) -> Result<usize, usize> {
        self.points.binary_search_by_key(&at, |&Point(x, _)| x)
    }

    pub fn eval(&self, at: impl Into<T>) -> T {
        let rnk = self.get_rnk(at.into());
        match rnk {
            Ok(rnk) => self.points[rnk].1,
            Err(rnk) => {
                if rnk == 0 {
                    self.points[0].1
                } else {
                    self.points[rnk - 1].1
                }
            }
        }
    }

    pub fn extend(&mut self, from_time: &T, value: &T) {
        let last_point = self.points.last_mut().unwrap();
        debug_assert!(*from_time >= last_point.0 - T::TOL);
        if abs(last_point.1 - *value) <= T::TOL {
            // The value is (by tolerance) the same as the last point, so we don't need to add a new point.
            return;
        }
        if abs(last_point.0 - *from_time) <= T::TOL {
            last_point.1 = *value;
        } else {
            self.points.push(Point(*from_time, *value));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{float::F64, num::Num, points};

    use super::PiecewiseConstant;

    #[test]
    pub fn it_evals_correctly() {
        let f: PiecewiseConstant<F64> = PiecewiseConstant::new(
            [-F64::INFINITY, F64::INFINITY],
            points![(1.0, 1.0), (2.0, 2.0)],
        );
        assert_eq!(f.eval(-1.0), 1.0);
        assert_eq!(f.eval(1.0), 1.0);
        assert_eq!(f.eval(1.5), 1.0);
        assert_eq!(f.eval(2.0), 2.0);
        assert_eq!(f.eval(3.0), 2.0);
    }

    #[test]
    pub fn it_extends_correctly() {
        let mut f: PiecewiseConstant<F64> =
            PiecewiseConstant::new([-F64::INFINITY, F64::INFINITY], points![(0.0, 0.0)]);
        f.extend(&1.0.into(), &2.0.into());

        assert_eq!(f.eval(-1.0), 0.0);
        assert_eq!(f.eval(0.9), 0.0);
        assert_eq!(f.eval(1.0), 2.0);
        f.extend(&(F64::from(1.0) + F64::TOL / F64::from(2.0)), &3.0.into());
        assert_eq!(f.eval(1.0), 3.0);

        f.extend(&3.0.into(), &3.0.into());
        assert_eq!(f.eval(1.0), 3.0);
        assert_eq!(f.eval(4.0), 3.0);
        assert_eq!(f.points.len(), 2)
    }
}
