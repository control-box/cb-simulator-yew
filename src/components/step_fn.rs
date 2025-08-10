use input_rs::yew::Input;
use yew::prelude::*;

use control_box::signal::step_fn::StepFunction;

#[derive(Properties, PartialEq)]
pub struct StepFunctionDialogProps {
    /// The state handle for managing the value of the input.
    pub handle: UseStateHandle<StepFunction<f64>>,
}

#[function_component(StepFunctionDialog)]
pub fn step_function_dialog(props: &StepFunctionDialogProps) -> Html {
    let updated = (*props.handle).clone();

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

    let updated = StepFunction::<f64> {
        pre_value: (*pre_value_handle).parse::<f64>().unwrap_or_default(),
        post_value: (*post_value_handle).parse::<f64>().unwrap_or_default(),
        step_time: (*step_time_handle).parse::<f64>().unwrap_or_default(),
    };

    props.handle.set(updated.clone());

    html! {
        <div>
       <form  class="flex flex-row">
            <div class="flex flex-col w-64">
                <label class="block text-sm mb-2 form-field w-64 text-gray-300 dark:text-gray-700
                " for="step_function_label"> { "Signal Type" } </label>
                <div id="step_function_label" class=" text-lg font-bold w-64"> { "Step Function"} </div>
            </div>
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
                label_class="block text-sm mb-2"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
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
                label_class="block text-sm mb-2"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
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
                label_class="block text-sm mb-2"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="error-text"
            />
        </form>
        </div>
    }
}
