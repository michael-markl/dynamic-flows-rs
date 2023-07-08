use std::{
    cmp::{max, min},
    iter::once,
    path::Path,
};

use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LabelAreaPosition},
    series::LineSeries,
    style::{ShapeStyle, RED, WHITE},
};

use crate::{num::Num, piecewise_linear::PiecewiseLinear};

pub fn plot<T: Num, P: AsRef<Path> + ?Sized>(pwl: PiecewiseLinear<T>, path: &P) {
    let drawing_area = BitMapBackend::new(path, (1024, 768)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let ((mut min_x, mut max_x), (mut min_y, mut max_y)) = {
        let mut min_y: T = T::INFINITY;
        let mut max_y: T = -T::INFINITY;
        for p in pwl.points.iter() {
            min_y = min(min_y, p.1.clone());
            max_y = max(max_y, p.1.clone());
        }
        let min_x = pwl.points[0].0;
        let max_x = pwl.points.last().unwrap().0;
        ((min_x, max_x), (min_y, max_y))
    };
    if min_x > pwl.domain.0 {
        min_x = if pwl.domain.0 > -T::INFINITY {
            pwl.domain.0
        } else {
            min_x - T::ONE
        };
        min_y = min(min_y, pwl.eval(min_x));
        max_y = max(max_y, pwl.eval(min_x));
    }
    if max_x < pwl.domain.1 {
        max_x = if pwl.domain.1 < T::INFINITY {
            pwl.domain.1
        } else {
            max_x + T::ONE
        };
        min_y = min(min_y, pwl.eval(max_x));
        max_y = max(max_y, pwl.eval(max_x));
    }

    let mut chart = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 100)
        .set_label_area_size(LabelAreaPosition::Bottom, 100)
        .build_cartesian_2d(
            min_x.to_f64()..max_x.to_f64(),
            (min_y.to_f64() - 1.)..(max_y.to_f64() + 1.),
        )
        .unwrap();
    chart.configure_mesh().draw().unwrap();
    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .draw()
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            once((min_x.to_f64(), pwl.eval(min_x).to_f64()))
                .chain(pwl.points.iter().map(|p| (p.0.to_f64(), p.1.to_f64())))
                .chain(once((max_x.to_f64(), pwl.eval(max_x).to_f64()))),
            ShapeStyle {
                color: RED.into(),
                filled: true,
                stroke_width: 2,
            },
        ))
        .unwrap();

    drawing_area.present().unwrap();
}
