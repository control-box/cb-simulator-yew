use yew::prelude::*;
use yew_plotly::plotly::common::Mode;
use yew_plotly::plotly::Plot;
use plotly::{Scatter, Layout, layout::Axis};
use yew_plotly::Plotly;
use ndarray::{Array, Ix1};
use plotly::ndarray::ArrayTraces;


#[derive(Properties, PartialEq)]
pub struct PlotlyExampleProps {
}

#[function_component(PlotlyExample)]
pub fn plotly_example(PlotlyExampleProps { }: &PlotlyExampleProps) -> Html {

    let n: usize = 11;
    let t: Array<f64, Ix1> = Array::range(0., 10., 10. / n as f64);
    let ys: Array<f64, Ix1> = t.iter().map(|v| (*v).powf(2.)).collect();

    let trace = Scatter::from_array(t, ys)
        .mode(Mode::LinesMarkers)
        .show_legend(true)
        .name("Scatter");


    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .title("<b>Line and Scatter Plot</b>".into())
        .x_axis(Axis::new()
            .title("time [ms]".into())
        );
    plot.set_layout(layout);


    html! { <Plotly plot={plot}/> }
}
