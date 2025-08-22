use input_rs::yew::Input;
use yew::prelude::*;
use log::debug;

use cb_controller::pid::PidSetpointRange;


#[derive(Properties, PartialEq)]
pub struct PidControllerOptionalInputDialogProps {
    pub config: Option<PidSetpointRange<f64>>,
    pub on_update: Callback<Option<PidSetpointRange<f64>>>,
}


#[function_component(PidControllerInputDialog)]
pub fn pid_controller_input_dialog(props: &PidControllerOptionalInputDialogProps) -> Html {

    debug!("PidControllerInputDialog - Entry: {:?}", props.config);

    let is_input_rangeed = use_state( || props.config.is_some());

    let input_range_change = {
        let is_input_rangeed = is_input_rangeed.clone();
        let emitter = props.on_update.clone();
        Callback::from(move |e: Event| {

            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            debug!("Input limitation {} Checkbox {}", *is_input_rangeed, input.checked(), );

            is_input_rangeed.set(input.checked());
            if !input.checked() { emitter.emit(None) }
        })
    };

    let input_change = {
        let emitter = props.on_update.clone();
        Callback::from(move |config: PidSetpointRange<f64>| {
            debug!("Input Change {:?} propagate", config );
            emitter.emit(Some(config));
        })
    };



    html! {
        <form  class="flex flex-row m-2 rounded border p-2 border-gray-400 dark:border-gray-600">

            <div class="flex flex-col w-40">
                <label for="open_loop_label" class="block mb-2 text-sm font-medium text-gray-300 dark:text-gray-700">
                    { "Setpoint Range" }
                </label>
                <div id="open_loop_label">
                    <label class="relative inline-flex items-center cursor-pointer">
                        <span class="ms-3 p-2 text-sm font-medium text-gray-900 dark:text-gray-300"> {"No"}</span>
                        <input type="checkbox" checked={*is_input_rangeed} onchange={input_range_change} class="sr-only peer"/>
                        <div class="relative w-11 h-6 bg-gray-200 rounded-full peer peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600"></div>
                        <span class="ms-3 p-2 text-sm font-medium text-gray-900 dark:text-gray-300"> {"Yes"}</span>
                    </label>
                </div>
            </div>

           if *is_input_rangeed {
                <PidControllerLimitedInputDialog config={
                    match props.config.clone() {
                        Some(c) => c,
                        None => PidSetpointRange::default(),
                    }
                } on_update={ input_change } />
           }
        </form>
    }
}


#[derive(Properties, PartialEq)]
pub struct PidControllerInputDialogProps {
    pub config: PidSetpointRange<f64>,
    pub on_update: Callback<PidSetpointRange<f64>>,
}

#[function_component(PidControllerLimitedInputDialog)]
pub fn pid_controller_limited_input_dialog(props: &PidControllerInputDialogProps) -> Html {

    fn always_valid(_s: String) -> bool {
        true
    }

    let input_config = props.config.clone();
    debug!("PidControllerLimitedInputDialog - Entry: {:?}", input_config);

    let input_min_ref = use_node_ref();
    let input_min_handle =  use_state(|| input_config.minimum().to_string());
    let input_min_valid_handle = use_state(|| true);

    let input_max_ref = use_node_ref();
    let input_max_handle =  use_state(|| input_config.maximum().to_string());
    let input_max_valid_handle = use_state(|| true);

    let off_band_output_ref = use_node_ref();
    let off_band_output_handle =  use_state(|| input_config.off_band_output.to_string());
    let off_band_output_valid_handle = use_state(|| true);

    let updated = input_config.clone()
        .range(
            (*input_min_handle).parse::<f64>().unwrap_or_default(),
            (*input_max_handle).parse::<f64>().unwrap_or_default(),
        )
        .out_of_band_output((*off_band_output_handle).parse::<f64>().unwrap_or_default())
        ;
    if updated != input_config {
        props.on_update.emit(updated);
    }


    html! {
        <div class="flex flex-row">

            <Input
                r#type="number"
                name="input_min"
                r#ref={input_min_ref}
                handle={input_min_handle}
                valid_handle={input_min_valid_handle}
                validate_function={always_valid}

                label="Minimum Input"
                required={true}
                error_message="Must be a not-negative number"
                class="form-field w-32 pl-2 pr-2"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="error-text"
            />

            <Input
                r#type="number"
                name="input_max"
                r#ref={input_max_ref}
                handle={input_max_handle}
                valid_handle={input_max_valid_handle}
                validate_function={always_valid}

                label="Maximum Input"
                required={true}
                error_message="Must be a not negative number"
                class="form-field w-32 pl-2 pr-2"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="error-text"
            />

            <Input
                r#type="number"
                name="off_band_output"
                r#ref={off_band_output_ref}
                handle={off_band_output_handle}
                valid_handle={off_band_output_valid_handle}
                validate_function={always_valid}

                label="Maximum Input"
                required={true}
                error_message="Must be a not negative number"
                class="form-field w-32 pl-2 pr-2"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="error-text"
            />

        </div>
    }
}