mod float;
mod num;
mod piecewise_linear;
mod plot;
mod point;

use crate::{float::F64, num::Num, point::Point};
use piecewise_linear::PiecewiseLinear;

fn main() {
    let f1: PiecewiseLinear<F64> = PiecewiseLinear::new(
        (-F64::INFINITY, F64::INFINITY),
        1.0,
        1.0,
        points![(1.0, 1.0)],
    );

    let f2: PiecewiseLinear<F64> = PiecewiseLinear::new(
        (-F64::INFINITY, F64::INFINITY),
        3.0,
        1.0,
        points![(-2.0, 1.0)],
    );

    let g = &f1 + &f2;

    println!("Evaluation");
    println!("g(-1)={}", g.eval(-1.0));
    println!("g: {:}", g);
    println!("g(-3)={}", g.eval(-3.0));
    plot::plot(g, "test.png");
}
