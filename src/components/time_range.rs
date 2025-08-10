use accordion_rs::yew::{Accordion, Item, List};
use accordion_rs::Size;
use input_rs::yew::Input;
use yew::prelude::*;

use control_box::signal::TimeRange;

#[derive(Properties, PartialEq)]
pub struct TimeRangeDialogProps {
    /// The state handle for managing the value of the input.
    pub handle: UseStateHandle<TimeRange>,
}

#[function_component(TimeRangeDialog)]
pub fn time_range_dialog(props: &TimeRangeDialogProps) -> Html {
    let updated = props.handle.clone();

    fn validate_sample_interval(sample_interval: String) -> bool {
        let st: f64 = sample_interval.parse::<f64>().unwrap_or(-1.0);
        st > 0.0
    }

    fn validate_start(start: String) -> bool {
        let st: f64 = start.parse::<f64>().unwrap_or(-1.0);
        st > 0.0
    }

    fn validate_end(end: String) -> bool {
        let st: f64 = end.parse::<f64>().unwrap_or(-1.0);
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
        updated.set_sampling_interval((*sample_interval_handle).parse::<f64>().unwrap_or_default());
    let updated = updated.set_start((*start_handle).parse::<f64>().unwrap_or_default());
    let updated = updated.set_end((*end_handle).parse::<f64>().unwrap_or_default());

    props.handle.set(updated.clone());

    html! {
       <div>
       <form  class="flex flex-row">
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
                class="form-field w-64"
                label_class="block text-sm mb-2"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600  rounded "
                error_class="text-red-800"
            />
            <Input
                r#type="number"
                name="start"
                r#ref={start_ref}
                handle={start_handle}
                valid_handle={start_valid_handle}
                validate_function={validate_start}
                label="Start Time [ms]"
                required={true}
                error_message="Must be smaller than End Time"
                class="form-field w-64"
                label_class="block text-sm mb-2"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
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
                class="form-field w-64"
                label_class="block text-sm mb-2"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="error-text"
            />
        </form>
        <p>

        </p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct AccordeonTimeRangeProps {
    /// The state handle for managing the value of the input.
    pub handle: UseStateHandle<TimeRange>,
}

#[function_component(AccordeonTimeRange)]
pub fn accordeon_time_range(props: &AccordeonTimeRangeProps) -> Html {
    let expand = use_state(|| false);
    let time_range = (*props.handle).clone();

    html! {
        <Accordion
            expand={expand}
            expanded={html! { " Time Range" }}
            collapsed={html! {<>
                 { "Time Range - Start:" } {time_range.start.to_string()}
                 { " End " } {time_range.end.to_string()}
                 { " Interval "} {time_range.sampling_interval.to_string()}
            </>}}
            size={Size::Custom("auto")}
            class="p-4 rounded border border-gray-400 dark:border-gray-600"
            expanded_class=" bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_class=" bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
        >
            <List>
                <Item>
                    <TimeRangeDialog handle={props.handle.clone()} />
                </Item>
            </List>
        </Accordion>

    }
}
