mod graph;
mod piecewise_linear;

use ordered_float::{NotNan, OrderedFloat};
use num_traits::{Float, FromPrimitive};
use piecewise_linear::{PiecewiseLinear, Point};

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


    println!("Evaluation: {}", f.eval(num(12.)));
}
