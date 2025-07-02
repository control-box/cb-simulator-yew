use accordion_rs::Size;
use accordion_rs::yew::{Accordion, Item, List};
use ndarray::{Array, Ix1};
use plotly::{Layout, Scatter, layout::Axis};
use yew::prelude::*;
use yew_plotly::Plotly;
use yew_plotly::plotly::Plot;
use yew_plotly::plotly::common::Mode;

use control_box::signal::{StepFunction, TimeRange, TimeSignal};

#[derive(Properties, PartialEq)]
pub struct TimeSignalProps {
    #[prop_or_default]
    pub range: TimeRange,
    #[prop_or_default]
    pub signal: StepFunction<f64>,
}

#[function_component(PlotTimeSignal)]
pub fn plotly_time_signal(props: &TimeSignalProps) -> Html {
    let time: Array<f64, Ix1> = props.range.collect();
    //let s = StepFunction::default();
    let signal: Array<f64, Ix1> = time
        .iter()
        .map(|v| props.signal.time_to_signal(*v))
        .collect();

    let mut plot = Plot::new();
    let trace = Scatter::from_array(time.clone(), signal)
        .mode(Mode::LinesMarkers)
        .show_legend(false)
        .name("Time signal");
    plot.add_trace(trace);

    let layout = Layout::new()
        .title("<b>Signal in Time Domain</b>".into())
        .x_axis(
            Axis::new().title("time [ms]".into()), // plotly 0.8.3 does not support From<String>
        )
        .y_axis(Axis::new().title("Signal Aplitude".into()));
    plot.set_layout(layout);

    html! {
        <Plotly plot={plot}/>
    }
}

// expanded_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
// collapsed_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
// class="w-full my-content-class bg-gray-500 p-4 rounded border-t border-gray-700"
#[function_component(AccordeonPlotTimeSignal)]
pub fn accordeon_time_signal(props: &TimeSignalProps) -> Html {
    let expand = use_state(|| true);

    html! {

        <Accordion
            expand={expand}
            expanded={html!
                { "Plot Signal" }
           }
            collapsed={html!
                { "Plot Signal: " }
            }
            expanded_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
            class="w-full my-content-class bg-gray-500 p-4 rounded border-t border-gray-700"
            size={Size::Custom("auto")}
        >
            <List>
                <Item>
                    <PlotTimeSignal range={props.range} signal={props.signal}/>
                </Item>
            </List>
        </Accordion>

    }
}
