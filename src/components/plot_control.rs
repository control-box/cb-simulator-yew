use accordion_rs::yew::{Accordion, Item, List};
use accordion_rs::Size;
use ndarray::{Array1};
use plotly::common::AxisSide;
use plotly::{layout::Axis, Layout, Scatter};
use yew::prelude::*;
use yew_plotly::plotly::common::{Mode, Title};
use yew_plotly::plotly::Plot;
use yew_plotly::Plotly;
use web_sys::HtmlSelectElement;
use log::info;

use crate::plant::named_element::NamedElement;
use crate::time_signal::named_time_signal::NamedTimeSignal;
use cb_simulation_util::signal::TimeRange;

use cb_controller::pid::PidController;

#[derive(Properties, PartialEq)]
pub struct ControlProps {
    #[prop_or_default]
    pub range: TimeRange,
    #[prop_or_default]
    pub signal: NamedTimeSignal<f64>,
    #[prop_or_default]
    pub element: NamedElement<f64>,
    #[prop_or_default]
    pub controller: PidController<f64>,
    #[prop_or_default]
    pub open_loop: bool,
}

#[function_component(PlotControl)]
pub fn plotly_time_signal(props: &ControlProps) -> Html {
    let time: Array1<f64> = props.range.collect();
    let signal = props.signal.signal.clone();
    let setpoint: Array1<f64> = time.iter().map(|v| signal.time_to_signal(*v)).collect();

    let mut element = props.element.element.clone();

    let mut pid = props.controller.clone();
    info!("Plot for PID Controller: {:?}", pid);

    let (control_variable, process_variable) = if props.open_loop {
        let y_no_feedback = 0.0; // open loop update
        // controller output u: aka control variable
        let u_open_loop: Array1<f64> = setpoint.iter().map(|sp| pid.update(y_no_feedback, *sp)).collect();
        // plant output y: aka process variable
        let y_open_loop: Array1<f64> = u_open_loop.iter().map(|v| element.transfer_td(*v)).collect();
        (u_open_loop, y_open_loop)
    } else {
        let dim = setpoint.len();
        let mut u = Array1::zeros(dim);
        let mut y = Array1::zeros(dim);
        for i in 0..(dim-1) {
            y[i] = element.transfer_td(u[i]);
            u[i+1] = pid.update(setpoint[i], y[i]);
        }
        ( u, y)
    };

    let mut plot = Plot::new();
    let setpoint_trace = Scatter::from_array(time.clone(), setpoint)
        .mode(Mode::LinesMarkers)
        .show_legend(true)
        .name("r: Setpoint");
    let process_variable_trace = Scatter::from_array(time.clone(), process_variable)
        .mode(Mode::LinesMarkers)
        .show_legend(true)
        .name("y: Process Variable");
    let control_variable_trace = Scatter::from_array(time.clone(), control_variable)
        .mode(Mode::LinesMarkers)
        .show_legend(true)
        .name("u: Control Variable")
        .y_axis("y2");


    plot.add_trace(setpoint_trace);
    plot.add_trace(control_variable_trace);
    plot.add_trace(process_variable_trace);

    let layout = Layout::new()
        .title("<b>Control Loop in Time Domain</b>".into())
        .x_axis(
            Axis::new().title("time [ms]".into()), // plotly 0.8.3 does not support From<String>
        )
        .y_axis(Axis::new().title("Setpoint Process Variable".into()))
        .y_axis2(
            Axis::new()
                .title(Title::from("Control Variable"))
                .overlaying("y")
                .side(AxisSide::Right),
        );
    plot.set_layout(layout);

    html! {
        <Plotly plot={plot}/>
    }
}

#[derive(Properties, PartialEq)]
pub struct AccordeonPlotControlProps {
    #[prop_or_default]
    pub range: TimeRange,
    pub signals: Vec<NamedTimeSignal<f64>>,
    pub elements: Vec<NamedElement<f64>>,
    pub controller: PidController<f64>,
}

#[function_component(AccordeonPlotControl)]
pub fn accordeon_time_signal(props: &AccordeonPlotControlProps) -> Html {
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

    let is_open_loop_checked = use_state(|| false);
    let on_open_loop_change = {
        let is_open_loop_checked = is_open_loop_checked.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            is_open_loop_checked.set(input.checked());
        })
    };

    html! {
        <Accordion
            expand={expand}
            expanded={html! { "Plot Control/Plant/Process" } }
            collapsed={html! { "Show Plot of Control/Plant/Process" } }
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
                    <div class="flex flex-col w-64">
                        <label for="open_loop_label" class="block mb-2 text-sm font-medium text-gray-300 dark:text-gray-700"> { "Control Loop" } </label>
                        <div id="open_loop_label">
                            <label class="relative inline-flex items-center cursor-pointer">
                                <span class="ms-3 p-2 text-sm font-medium text-gray-900 dark:text-gray-300"> {"Close"}</span>
                                <input type="checkbox" checked={*is_open_loop_checked} onchange={on_open_loop_change} class="sr-only peer"/>
                                <div class="relative w-11 h-6 bg-gray-200 rounded-full peer peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600"></div>
                                <span class="ms-3 p-2 text-sm font-medium text-gray-900 dark:text-gray-300"> {"Open"}</span>
                            </label>
                        </div>
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
                                            <PlotControl range={props.range.clone()} signal={signal.clone()} element={element.clone()} controller={props.controller.clone()} open_loop={ *is_open_loop_checked }/>
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
