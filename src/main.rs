mod graph;
mod piecewise_linear;


use ordered_float::{OrderedFloat};

use piecewise_linear::{PiecewiseLinear, Point};
use crate::piecewise_linear::CustomNum;

impl CustomNum for OrderedFloat<f64> {
    const EXACT_ARITHMETIC: bool = false;
    const TOL: Self = flt(1e-9);
}

const fn flt(val: f64) -> OrderedFloat<f64> {
    OrderedFloat(val)
}

fn main() {
    let f1: PiecewiseLinear<OrderedFloat<f64>> = PiecewiseLinear {
        domain: (flt(-f64::INFINITY), flt(f64::INFINITY)),
        first_slope: flt(1.),
        last_slope: flt(1.),
        points: vec![Point(flt(1.), flt(1.))],
    };

    let f2: PiecewiseLinear<OrderedFloat<f64>> = PiecewiseLinear {
        domain: (flt(-3.), flt(-1.)),
        first_slope: flt(3.),
        last_slope: flt(1.),
        points: vec![Point(flt(-2.), flt(1.))],
    };

    let g = &f1 + &f2;

    println!("Evaluation: {}", g.eval(flt(-1.)));
    println!("g: {:#?}", g);
}
