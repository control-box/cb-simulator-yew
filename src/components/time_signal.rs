use input_rs::yew::Input;
use ndarray::{Array, Ix1};
use plotly::{Layout, Scatter, layout::Axis};
use yew::prelude::*;
use yew_accordion::{Accordion, AccordionButton, AccordionItem};
use yew_plotly::Plotly;
use yew_plotly::plotly::Plot;
use yew_plotly::plotly::common::Mode;

use control_box::signal::{StepFunction, TimeRange, TimeSignal};

#[derive(Properties, PartialEq)]
pub struct TimeRangeDialogProps {
    #[prop_or_default]
    range: TimeRange,
    /// The state handle for managing the value of the input.
    pub handle: UseStateHandle<TimeRange>,
}

#[function_component(TimeRangeDialog)]
pub fn time_range_dialog(props: &TimeRangeDialogProps) -> Html {
    let updated = props.range.clone();

    fn validate_sample_interval(sample_interval: String) -> bool {
        let st: f32 = sample_interval.parse::<f32>().unwrap_or(-1.0);
        st > 0.0
    }

    fn validate_start(start: String) -> bool {
        let st: f32 = start.parse::<f32>().unwrap_or(-1.0);
        st > 0.0
    }

    fn validate_end(end: String) -> bool {
        let st: f32 = end.parse::<f32>().unwrap_or(-1.0);
        st > 0.0
    }

    let sample_interval_ref = use_node_ref();
    let sample_interval_handle = use_state(|| updated.sampling_interval.to_string());
    let sample_interval_valid_handle = use_state(|| true);

    let start_ref = use_node_ref();
    let start_handle = use_state(|| updated.start.to_string());
    let start_valid_handle = use_state(|| true);

    let end_ref = use_node_ref();
    let end_handle = use_state(|| updated.end.to_string());
    let end_valid_handle = use_state(|| true);

    let updated =
        updated.set_sampling_interval((*sample_interval_handle).parse::<f32>().unwrap_or_default());
    let updated = updated.set_start((*start_handle).parse::<f32>().unwrap_or_default());
    let updated = updated.set_end((*end_handle).parse::<f32>().unwrap_or_default());

    props.handle.set(updated.clone());

    html! {
       <div>
       <form>
            <Input
                r#type="number"
                min="1"
                name="sample_interval"
                r#ref={sample_interval_ref}
                handle={sample_interval_handle}
                valid_handle={sample_interval_valid_handle}
                validate_function={validate_sample_interval}

                label="Sample interval [ms]"
                required={true}
                error_message="Must be a positive number"
                class="form-field"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="text-red-800"
            />
            <Input
                r#type="number"
                name="start"
                min="1"
                r#ref={start_ref}
                handle={start_handle}
                valid_handle={start_valid_handle}
                validate_function={validate_start}
                label="Start Time [ms]"
                required={true}
                error_message="Must be smaller than End Time"
                class="form-field"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="error-text"
            />
            <Input
                r#type="number"
                name="end"
                r#ref={end_ref}
                handle={end_handle}
                valid_handle={end_valid_handle}
                validate_function={validate_end}
                label="End Time [ms]"
                required={true}
                error_message="Must greater than Start Time"
                class="form-field"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="error-text"
            />
        </form>
        <p>
            { "Start:" } {updated.start.to_string()} { " End " } {updated.end.to_string()} { " Interval "} {updated.sampling_interval.to_string()}
        </p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TimeSignalProps {
    #[prop_or_default]
    range: TimeRange,
}

#[function_component(PlotTimeSignal)]
pub fn plotly_time_signal(TimeSignalProps { range }: &TimeSignalProps) -> Html {
    let time: Array<f32, Ix1> = range.collect();
    let step_fn = StepFunction::default().pre(2.0).post(3.0).step(2.5);
    let signal: Array<f32, Ix1> = time.iter().map(|v| step_fn.time_to_signal(*v)).collect();

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

#[derive(Properties, PartialEq)]
pub struct AccordeonTimeSignalProps {}

#[function_component(AccordeonTimeSignal)]
pub fn accordeon_time_signal(_props: &AccordeonTimeSignalProps) -> Html {
    let time_range_handle = use_state(TimeRange::default);
    let time_range = (*time_range_handle).clone();

    html! {
        <Accordion
            expanded_element={html! {<AccordionButton class={"bg-blue-500 text-white p-2 rounded"}>
                { "Signal in Time Domain" }
            </AccordionButton>}}
            collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded"}>
                { "Signal in Time Domain" }
            </AccordionButton>}}

            aria_controls="example-accordion"
            container_class="my-custom-class bg-gray-800 p-4 rounded border border-gray-400"
            expanded_element_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
            content_container_class="my-content-class bg-gray-500 p-4 rounded border-t border-gray-700"
        >
            <ul>
                <AccordionItem
                    item_class="my-list-item-class border-b p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
                >
                    < TimeRangeDialog range={TimeRange::default()} handle={time_range_handle} />
                </AccordionItem>
                <AccordionItem
                    item_class="my-list-item-class border-b p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
                >
                    <PlotTimeSignal range={time_range}/>
                </AccordionItem>
            </ul>
        </Accordion>

    }
}
