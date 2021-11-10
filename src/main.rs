mod graph;
mod piecewise_linear;


use ordered_float::{OrderedFloat};

use piecewise_linear::{PiecewiseLinear, Point};
use crate::piecewise_linear::CustomNum;

impl CustomNum for OrderedFloat<f64> {}

fn num(val: f64) -> OrderedFloat<f64> {
    OrderedFloat(val)
}

fn main() {
    let f: PiecewiseLinear<OrderedFloat<f64>> = PiecewiseLinear {
        domain: (num(-f64::INFINITY), num(f64::INFINITY)),
        first_slope: num(1.),
        last_slope: num(1.),
        points: vec![Point(num(1.), num(1.))],
    };

    let g = &f + &f;

    println!("Evaluation: {}", g.eval(num(12.)));
    println!("g: {:#?}", g);
}
