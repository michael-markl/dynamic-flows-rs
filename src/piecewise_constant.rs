use crate::num::Num;
use crate::point::Point;

#[derive(Debug)]
pub struct PiecewiseConstant<T: Num> {
    pub domain: (T, T),
    pub points: Vec<Point<T>>, // TODO: Maybe use a NonEmptyVec here
}

impl<T: Num> PiecewiseConstant<T> {
    pub fn new(domain: (impl Into<T>, impl Into<T>), points: Vec<Point<T>>) -> Self {
        let domain = (domain.0.into(), domain.1.into());
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

        Self { domain, points }
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
}

#[cfg(test)]
mod tests {
    use crate::{float::F64, num::Num, points};

    use super::PiecewiseConstant;

    #[test]
    pub fn it_evals_correctly() {
        let f: PiecewiseConstant<F64> = PiecewiseConstant::new(
            (-F64::INFINITY, F64::INFINITY),
            points![(1.0, 1.0), (2.0, 2.0)],
        );
        assert_eq!(f.eval(-1.0), 1.0);
        assert_eq!(f.eval(1.0), 1.0);
        assert_eq!(f.eval(1.5), 1.0);
        assert_eq!(f.eval(2.0), 2.0);
        assert_eq!(f.eval(3.0), 2.0);
    }
}
