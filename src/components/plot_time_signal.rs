use accordion_rs::yew::{Accordion, Item, List};
use accordion_rs::Size;
use ndarray::{Array, Ix1};
use plotly::{layout::Axis, Layout, Scatter};
use yew::prelude::*;
use yew_plotly::plotly::common::Mode;
use yew_plotly::plotly::Plot;
use yew_plotly::Plotly;

use web_sys::HtmlSelectElement;

use control_box::signal::{NamedTimeSignal, TimeRange};

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
// expanded_class=" bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
// collapsed_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
// class="w-full my-content-class bg-gray-500 p-4 rounded border-t border-gray-700"
#[function_component(AccordeonPlotTimeSignal)]
pub fn accordeon_time_signal(props: &AccordeonPlotTimeSignalProps) -> Html {
    let expand = use_state(|| false);
    // Collect signal names for the select dropdown
    let signal_names = props
        .signals
        .iter()
        .enumerate()
        .map(|(index, signal)| {
            html! {
                        <option value={index.to_string()}
                        selected={index == 0} // if the list get changed always the first element is selected
            >
                            { signal.name.clone() }
                        </option>
                    }
        })
        .collect::<Vec<Html>>();

    let initial_selected = if !props.signals.is_empty() {
        "0".to_string()
    } else {
        "".to_string()
    };
    let selected = use_state(|| initial_selected);

    let selected_clone = selected.clone();
    let on_change = Callback::from(move |event: Event| {
        let target = event.target_dyn_into::<HtmlSelectElement>();
        if let Some(select) = target {
            selected_clone.set(select.value());
        }
    });

    html! {
        <Accordion
            expand={expand}
            expanded={html! { "Plot Time Signal" } }
            collapsed={html! { "Plot Time Signal " } }
            expanded_class=" bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_class="bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
            class="w-full p-4 rounded border border-gray-400 dark:border-gray-600"
            size={Size::Custom("auto")}
        >
            <List>
                <Item>
                    <div class=" mx-auto">
                        <label for="signal_type_label" class="block mb-2 text-sm font-medium text-gray-300 dark:text-gray-700"> { "Select Signal by Name" } </label>
                        <select name={"signal"} onchange={ on_change}
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        id="signal_type_label">
                            { signal_names }
                        </select>
                    </div>
                </Item>
                {
                    if props.signals.is_empty() {
                        html! { <Item>{"No signals available"}</Item> }
                    } else {
                        let index = selected.parse::<usize>().unwrap_or(0);

                        if let Some(signal) = props.signals.get(index) {
                            html! {
                                <Item>
                                    <PlotTimeSignal range={props.range.clone()} signal={signal.clone()} />
                                </Item>
                            }
                        } else {
                            html! { <Item>{"No signals available"}</Item> }
                        }
                    }
                }
            </List>
        </Accordion>
    }
}
