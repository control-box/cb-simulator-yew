use accordion_rs::yew::{Accordion, Item, List};
use accordion_rs::Size;
use ndarray::{Array, Ix1};
use plotly::common::AxisSide;
use plotly::{layout::Axis, Layout, Scatter};
use yew::prelude::*;
use yew_plotly::plotly::common::{Mode, Title};
use yew_plotly::plotly::Plot;
use yew_plotly::Plotly;

use web_sys::HtmlSelectElement;

use crate::components::plant::named_element::NamedElement;
use control_box::signal::{NamedTimeSignal, TimeRange};

#[derive(Properties, PartialEq)]
pub struct ElementProps {
    #[prop_or_default]
    pub range: TimeRange,
    #[prop_or_default]
    pub signal: NamedTimeSignal<f64>,
    #[prop_or_default]
    pub element: NamedElement<f64>,
}

#[function_component(PlotElement)]
pub fn plotly_time_signal(props: &ElementProps) -> Html {
    let time: Array<f64, Ix1> = props.range.collect();
    let signal = props.signal.signal.clone();
    let input: Array<f64, Ix1> = time.iter().map(|v| signal.time_to_signal(*v)).collect();
    let mut element = props.element.element.clone();
    let output: Array<f64, Ix1> = input.iter().map(|v| element.transfer_td(*v)).collect();

    let mut plot = Plot::new();
    let input_trace = Scatter::from_array(time.clone(), input)
        .mode(Mode::LinesMarkers)
        .show_legend(true)
        .name("Input signal");
    let output_trace = Scatter::from_array(time.clone(), output)
        .mode(Mode::LinesMarkers)
        .show_legend(true)
        .name("Output signal")
        .y_axis("y2");

    plot.add_trace(input_trace);
    plot.add_trace(output_trace);

    let layout = Layout::new()
        .title("<b>Transfer function of Element in Time Domain</b>".into())
        .x_axis(
            Axis::new().title("time [ms]".into()), // plotly 0.8.3 does not support From<String>
        )
        .y_axis(Axis::new().title("Input Amplitude".into()))
        .y_axis2(
            Axis::new()
                .title(Title::from("Output Amplitude"))
                .overlaying("y")
                .side(AxisSide::Right),
        );
    plot.set_layout(layout);

    html! {
        <Plotly plot={plot}/>
    }
}

#[derive(Properties, PartialEq)]
pub struct AccordeonPlotElementProps {
    #[prop_or_default]
    pub range: TimeRange,
    #[prop_or_default]
    pub signals: Vec<NamedTimeSignal<f64>>,
    #[prop_or_default]
    pub elements: Vec<NamedElement<f64>>,
}

#[function_component(AccordeonPlotElement)]
pub fn accordeon_time_signal(props: &AccordeonPlotElementProps) -> Html {
    let expand = use_state(|| false);

    let signal_names = props
        .signals
        .iter()
        .enumerate()
        .map(|(index, signal)| {
            html! {
                <option value={index.to_string()}
                // if the list get changed always the first element is selected
                selected={index == 0}>
                    { signal.name.clone() }
                </option>
            }
        })
        .collect::<Vec<Html>>();
    let element_names = props
        .elements
        .iter()
        .enumerate()
        .map(|(index, element)| {
            html! {
                <option value={index.to_string()}
                // if the list get changed always the first element is selected
                selected={index == 0}>
                    { element.name.clone() }
                </option>
            }
        })
        .collect::<Vec<Html>>();

    let initial_selected_signal = if !props.signals.is_empty() {
        "0".to_string()
    } else {
        "".to_string()
    };
    let selected_signal = use_state(|| initial_selected_signal);
    let initial_selected_element = if !props.elements.is_empty() {
        "0".to_string()
    } else {
        "".to_string()
    };
    let selected_element = use_state(|| initial_selected_element);

    let selected_signal_clone = selected_signal.clone();
    let on_signal_change = Callback::from(move |event: Event| {
        let target = event.target_dyn_into::<HtmlSelectElement>();
        if let Some(select) = target {
            selected_signal_clone.set(select.value());
        }
    });
    let selected_element_clone = selected_element.clone();
    let on_element_change = Callback::from(move |event: Event| {
        let target = event.target_dyn_into::<HtmlSelectElement>();
        if let Some(select) = target {
            selected_element_clone.set(select.value());
        }
    });

    html! {
        <Accordion
            expand={expand}
            expanded={html! { "Plot Element/Plant/Process" } }
            collapsed={html! { "Show Plot of Element/Plant/Process" } }
            expanded_class=" bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_class="bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
            class="w-full p-4 rounded border border-gray-400 dark:border-gray-600"
            size={Size::Custom("auto")}
        >
            <List>
                <Item class="flex flex-row">
                    <div class="flex flex-col w-64">
                        <label for="signal_type_label" class="block mb-2 text-sm font-medium text-gray-300 dark:text-gray-700"> { "Select Signal by Name" } </label>
                        <select name={"signal"} onchange={on_signal_change}
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        id="signal_type_label">
                            { signal_names }
                        </select>
                    </div>
                    <div class="flex flex-col w-64">
                        <label for="element_label" class="block mb-2 text-sm font-medium text-gray-300 dark:text-gray-700"> { "Select Element by Name" } </label>
                        <select name={"signal"} onchange={on_element_change}
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        id="element_label">
                            { element_names }
                        </select>
                    </div>
                </Item>
                {
                    if props.signals.is_empty() {
                        html! { <Item>{"No signals available"}</Item> }
                    } else {
                        let index = selected_signal.parse::<usize>().unwrap_or(0);

                        if let Some(signal) = props.signals.get(index) {
                            if props.elements.is_empty() {
                                html! { <Item>{"No elements available"}</Item> }
                            } else {
                                let index = selected_element.parse::<usize>().unwrap_or(0);
                                if let Some(element) = props.elements.get(index) {
                                    html! {
                                        <Item>
                                            <PlotElement range={props.range.clone()} signal={signal.clone()} element={element.clone()} />
                                        </Item>
                                    }
                                } else {
                                    html! { <Item>{"No elements available"}</Item> }
                                }
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
