use input_rs::yew::Input;
use yew::prelude::*;

use control_box::signal::impulse_fn::ImpulseFunction;

#[derive(Properties, PartialEq)]
pub struct ImpulseFunctionDialogProps {
    /// The state handle for managing the value of the input.
    pub handle: UseStateHandle<ImpulseFunction<f64>>,
}

#[function_component(ImpulseFunctionDialog)]
pub fn impulse_function_dialog(props: &ImpulseFunctionDialogProps) -> Html {
    let updated = (*props.handle).clone();

    fn always_valid(_s: String) -> bool {
        true
    }

    let out_value_ref = use_node_ref();
    let out_value_handle = use_state(|| updated.out_value.to_string());
    let out_value_valid_handle = use_state(|| true);

    let in_value_ref = use_node_ref();
    let in_value_handle = use_state(|| updated.in_value.to_string());
    let in_value_valid_handle = use_state(|| true);

    let start_ref = use_node_ref();
    let start_handle = use_state(|| updated.start_time.to_string());
    let start_valid_handle = use_state(|| true);

    let duration_ref = use_node_ref();
    let duration_handle = use_state(|| updated.duration.to_string());
    let duration_valid_handle = use_state(|| true);

    let updated = ImpulseFunction::<f64> {
        out_value: (*out_value_handle).parse::<f64>().unwrap_or_default(),
        in_value: (*in_value_handle).parse::<f64>().unwrap_or_default(),
        start_time: (*start_handle).parse::<f64>().unwrap_or_default(),
        duration:(*duration_handle).parse::<f64>().unwrap_or_default(),
    };

    props.handle.set(updated.clone());

    html! {
        <div>
       <form  class="flex flex-row">
            <label class="block text-sm text-gray-300 mb-2 form-field w-64" for="step_function_label"> { "Impulse Function" } </label>
            <Input
                r#type="number"
                name="out_value"
                r#ref={out_value_ref}
                handle={out_value_handle}
                valid_handle={out_value_valid_handle}
                validate_function={always_valid}

                label="Base level"
                required={true}
                error_message="Must be a number"
                class="form-field w-64"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="text-red-800"
            />
            <Input
                r#type="number"
                name="in_value"
                r#ref={in_value_ref}
                handle={in_value_handle}
                valid_handle={in_value_valid_handle}
                validate_function={always_valid}

                label="Impulse Amplitude"
                required={true}
                error_message="Must be a number"
                class="form-field w-64"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="error-text"
            />
            <Input
                r#type="number"
                name="start"
                r#ref={start_ref}
                handle={start_handle}
                valid_handle={start_valid_handle}
                validate_function={always_valid}

                label="Time where the Impuse starts [ms]"
                required={true}
                error_message="Must be a number"
                class="form-field w-64"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="error-text"
            />
            <Input
                r#type="number"
                name="duration"
                r#ref={duration_ref}
                handle={duration_handle}
                valid_handle={duration_valid_handle}
                validate_function={always_valid}

                label="Duration of the Impuse [ms]"
                required={true}
                error_message="Must be a positive number"
                class="form-field w-64"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="error-text"
            />
        </form>
        </div>
    }
}
