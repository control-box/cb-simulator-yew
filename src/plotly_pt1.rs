use yew::prelude::*;
use yew_plotly::plotly::common::{
    Mode,
    DashType,
};
use yew_plotly::plotly::Plot;
use plotly::{Scatter,
        Layout,

        layout::{Axis,
            Shape,
            ShapeLine,
            ShapeType,

        },
        color::{
            NamedColor,
        },


    };
use yew_plotly::Plotly;
use ndarray::{Array, Ix1};
use plotly::ndarray::ArrayTraces;

use control_box::pt1::PT1;

#[derive(Properties, PartialEq)]
pub struct PlotlyPT1Props {
}

#[function_component(PlotlyPT1)]
pub fn plotly_pt1(PlotlyPT1Props { }: &PlotlyPT1Props) -> Html {

    let n: usize = 111;
    let t: Array<f64, Ix1> = Array::range(-10., 100., 1. / n as f64);
    let u: Array<f64, Ix1> = t.iter().map(|v| if *v  < 0. { 0. } else { 1.}).collect();

    let mut pt1 = PT1::<f64>::new(0.001, 0.005, 0.6);
    let y: Array<f64, Ix1> = u.iter().map(|v| pt1.transfer(*v)).collect();

    let mut plot = Plot::new();
    let trace = Scatter::from_array(t.clone(), u)
        .mode(Mode::LinesMarkers)
        .show_legend(true)
        .name("Step function stimulus");
    plot.add_trace(trace);
    let trace = Scatter::from_array(t, y)
        .mode(Mode::LinesMarkers)
        .show_legend(true)
        .name("PT1 Response T1 = 5 ms, Kp = =.5");
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .title("<b>PT1 Element</b>".into())
        .x_axis(Axis::new()
            .title("time [ms]".into())
        );
            layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(1)
            .y0(0)
            .x1(5)
            .y1(2)
            .line(ShapeLine::new().color(NamedColor::RoyalBlue).width(3.)),
    );

    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(2)
            .y0(2)
            .x1(5)
            .y1(2)
            .line(
                ShapeLine::new()
                    .color(NamedColor::LightSeaGreen)
                    .width(3.)
                    .dash(DashType::DashDot),
            ),
    );
    plot.set_layout(layout);


    html! { <Plotly plot={plot}/> }
}
