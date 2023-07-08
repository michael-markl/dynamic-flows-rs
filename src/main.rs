mod float;
mod graph;
mod num;
mod piecewise_linear;
mod plot;

use crate::{float::F64, num::Num};
use piecewise_linear::{PiecewiseLinear, Point};

fn main() {
    let f1: PiecewiseLinear<F64> = PiecewiseLinear::new(
        (-F64::INFINITY, F64::INFINITY),
        1.0.into(),
        1.0.into(),
        vec![Point(1.0.into(), 1.0.into())],
    );

    let f2: PiecewiseLinear<F64> = PiecewiseLinear::new(
        (-F64::INFINITY, F64::INFINITY),
        3.0.into(),
        1.0.into(),
        vec![Point((-2.0).into(), 1.0.into())],
    );

    let g = &f1 + &f2;

    println!("Evaluation: {}", g.eval((-1.0).into()));
    println!("g: {:}", g);
    println!("g(-3) {}", g.eval((-3.0).into()));
    plot::plot(g, "test.png");
}
