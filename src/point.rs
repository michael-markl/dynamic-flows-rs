use std::fmt::{Display, Formatter};

use crate::num::Num;

#[derive(Debug, Clone)]
pub struct Point<T: Num>(pub T, pub T);

impl<T: Num> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:}, {:})", self.0, self.1)
    }
}

impl<T: Num> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[macro_export]
macro_rules! points {
    ( $( $x:expr ),+ ) => {
        {
            use $crate::point::Point;
            vec!( $( Point($x.0.into(), $x.1.into()) ),* )
        }
    };
}
