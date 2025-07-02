use accordion_rs::Size;
use accordion_rs::yew::{Accordion, Item, List};
use input_rs::yew::Input;
use yew::prelude::*;

use std::vec::Vec;

use control_box::signal::*;

#[derive(Properties, PartialEq)]
pub struct AccordeonTimeSignalsProps {
    pub signals: UseStateHandle<Vec<NamedTimeSignal<f64>>>,
    pub on_add: Callback<NamedTimeSignal<f64>>,
    // pub on_remove: Callback<String>,
}

#[function_component(AccordeonTimeSignals)]
pub fn accordeon_time_signals(props:  &AccordeonTimeSignalsProps) -> Html {
    let expand = use_state(|| true);
    let on_add = props.on_add.clone();
    let on_add_click = Callback::from(move |_| {
        on_add.emit(NamedTimeSignal::<f64>::default());
    });

    let signals = (*props.signals).clone().iter().map(|signal| {

        html! {
            <Item>
                <button
                    class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Remove Signal"
                >
                <i class="fa fa-minus"></i> { "Remove"}
                </button>
                { format!("{}", signal) }
            </Item>
        }
    }).collect::<Html>();



    html! {
        <Accordion
            expand={expand}
            expanded={html! { " Times Signals" }}
            collapsed={html! {<>
                 { "Time Signals:" }
            </>}}
            size={Size::Custom("auto")}
            class="my-custom-class bg-gray-800 p-4 rounded border border-gray-400"
            expanded_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
        >
            <List>
                { signals }
                <Item>
                  <button onclick={on_add_click}
                    class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Add a signal"
                  >
                    <i class="fa fa-plus"></i> { "Add"}
                  </button>
                </Item>
            </List>
        </Accordion>

    }
}

#[derive(Properties, PartialEq)]
pub struct StepFunctionDialogProps {
    #[prop_or_default]
    pub step_fn: StepFunction<f64>,
    /// The state handle for managing the value of the input.
    pub handle: UseStateHandle<StepFunction<f64>>,
}

#[function_component(StepFunctionDialog)]
pub fn step_function_dialog(props: &StepFunctionDialogProps) -> Html {
    let mut updated = props.step_fn.clone();

    fn always_valid(_s: String) -> bool {
        true
    }

    let pre_value_ref = use_node_ref();
    let pre_value_handle = use_state(|| updated.pre_value.to_string());
    let pre_value_valid_handle = use_state(|| true);

    let post_value_ref = use_node_ref();
    let post_value_handle = use_state(|| updated.post_value.to_string());
    let post_value_valid_handle = use_state(|| true);

    let step_time_ref = use_node_ref();
    let step_time_handle = use_state(|| updated.step_time.to_string());
    let step_time_valid_handle = use_state(|| true);

    updated.step_time = (*step_time_handle).parse::<f64>().unwrap_or_default();
    updated.pre_value = (*pre_value_handle).parse::<f64>().unwrap_or_default();
    updated.post_value = (*post_value_handle).parse::<f64>().unwrap_or_default();

    props.handle.set(updated.clone());

    html! {
        <div>
       <form  class="flex flex-row">
            <Input
                r#type="number"
                name="pre_value"
                r#ref={pre_value_ref}
                handle={pre_value_handle}
                valid_handle={pre_value_valid_handle}
                validate_function={always_valid}

                label="Value prior to the step"
                required={true}
                error_message="Must be a number"
                class="form-field w-64"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="text-red-800"
            />
            <Input
                r#type="number"
                name="post_value"
                r#ref={post_value_ref}
                handle={post_value_handle}
                valid_handle={post_value_valid_handle}
                validate_function={always_valid}

                label="Value after the step"
                required={true}
                error_message="Must be a number"
                class="form-field w-64"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="error-text"
            />
            <Input
                r#type="number"
                name="step_time"
                r#ref={step_time_ref}
                handle={step_time_handle}
                valid_handle={step_time_valid_handle}
                validate_function={always_valid}

                label="Time where the step happens [ms]"
                required={true}
                error_message="Must greater than post_value Time"
                class="form-field w-64"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="error-text"
            />
        </form>
        </div>
    }
}

// https://stackoverflow.com/questions/42056422/using-any-with-traits-in-rust Any traits for reflexion

#[derive(Properties, PartialEq)]
pub struct TimeSignalDialogProps {
    #[prop_or_default]
    signal: NamedTimeSignal<f64>,
    /// The state handle for managing the value of the input.
    pub handle: UseStateHandle<NamedTimeSignal<f64>>,
}

#[function_component(TimeSignalDialog)]
pub fn time_signal_dialog(props: &TimeSignalDialogProps) -> Html {
    let updated: NamedTimeSignal<f64> = props.signal.clone();

    let name_ref = use_node_ref();
    let name_handle = use_state(|| updated.name.to_string());
    let name_valid_handle = use_state(|| true);

    fn always_valid(_s: String) -> bool {
        true
    }
    // !todo - Extract signal type from named signal trait object
    let step_fn_handle = use_state(StepFunction::<f64>::default);
    let signal = (*step_fn_handle).clone();

    props.handle.set(updated.clone());

    html! {
        <div>
       <form  class="flex flex-row">
            <Input
                r#type="text"
                name="name"
                r#ref={name_ref}
                handle={name_handle}
                valid_handle={name_valid_handle}
                validate_function={always_valid}

                label="Signal Name"
                required={true}
                error_message="must be a word"
                class="form-field w-64"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="text-red-800"
            />
        <label for={"signal_type"}> {"Select a signal:"}</label>
          <select id={"signal_type"} name={"signal_type"}>
            <option value={"step"}>{"Step Function"}</option>

            <option value={"white_noise"}>{"White Noise"}</option>
            <option value={"superposition"}>{"Superposition of two Signals"}</option>
            </select>
        </form>

       <StepFunctionDialog step_fn={signal} handle={step_fn_handle.clone()} />

        </div>
    }
}
