use accordion_rs::Size;
use accordion_rs::yew::{Accordion, Item, List};
use ndarray::{Array, Ix1};
use plotly::{Layout, Scatter, layout::Axis};
use yew::prelude::*;
use yew_plotly::Plotly;
use yew_plotly::plotly::Plot;
use yew_plotly::plotly::common::Mode;

use selectrs::yew::{Select, Option, Group};

use log::info;

use control_box::signal::{NamedTimeSignal, TimeRange, TimeSignal};

#[derive(Properties, PartialEq)]
pub struct TimeSignalProps {
    #[prop_or_default]
    pub range: TimeRange,
    #[prop_or_default]
    pub signal: NamedTimeSignal<f64>,
}

#[function_component(PlotTimeSignal)]
pub fn plotly_time_signal(props: &TimeSignalProps) -> Html {
    let time: Array<f64, Ix1> = props.range.collect();
    let signal_struct = props.signal.signal.clone();
    let signal: Array<f64, Ix1> = time
        .iter()
        .map(|v| signal_struct.time_to_signal(*v))
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
        .y_axis(Axis::new().title("Signal Amplitude".into()));
    plot.set_layout(layout);

    html! {
        <Plotly plot={plot}/>
    }
}

#[derive(Properties, PartialEq)]
pub struct AccordeonPlotTimeSignalProps {
    #[prop_or_default]
    pub range: TimeRange,
    #[prop_or_default]
    pub signals: Vec<NamedTimeSignal<f64>>,
}
// expanded_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
// collapsed_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
// class="w-full my-content-class bg-gray-500 p-4 rounded border-t border-gray-700"
#[function_component(AccordeonPlotTimeSignal)]
pub fn accordeon_time_signal(props: &AccordeonPlotTimeSignalProps) -> Html {
    let expand = use_state(|| true);

    let signal_names =  props.signals.iter().map(|signal| {
        html! {
            <option value={signal.name.clone()}>
                { signal.name.clone() }
            </option>
        }
    }).collect::<Html>();


    html! {
        <Accordion
            expand={expand}
            expanded={html! { "Select Time Signal" } }
            collapsed={html! { "Plot Time Signal: " } }
            expanded_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
            class="w-full my-content-class bg-gray-500 p-4 rounded border-t border-gray-700"
            size={Size::Custom("auto")}
        >
            <List>
                <Item>
                    <select name={"signal"} onchange={
                        Callback::from(|value| info!("selection change: {:?}", value))}
                    >
                        { signal_names }

                    </select>
                </Item>
                {
                    if props.signals.len() == 0 {
                        html! { <Item>{"No signals available"}</Item> }
                    } else {
                        html! {
                            <Item>
                                <PlotTimeSignal range={props.range.clone()} signal={props.signals[0].clone()} />
                            </Item>
                        }
                    }
                }
            </List>
        </Accordion>
    }
}
