use input_rs::yew::Input;
use yew::prelude::*;
use log::debug;

use cb_controller::pid::PidOutputLimit;


#[derive(Properties, PartialEq)]
pub struct PidControllerOptionalOutputDialogProps {
    pub config: Option<PidOutputLimit<f64>>,
    pub on_update: Callback<Option<PidOutputLimit<f64>>>,
}


#[function_component(PidControllerOutputDialog)]
pub fn pid_controller_output_dialog(props: &PidControllerOptionalOutputDialogProps) -> Html {

    debug!("PidControllerOutputDialog - Entry: {:?}", props.config);

    let is_output_limited = use_state( || props.config.is_some());

    let output_limit_change = {
        let is_output_limited = is_output_limited.clone();
        let emitter = props.on_update.clone();
        Callback::from(move |e: Event| {

            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            debug!("Output limitation {} Checkbox {}", *is_output_limited, input.checked(), );

            is_output_limited.set(input.checked());
            if !input.checked() { emitter.emit(None) }
        })
    };

    let output_change = {
        let emitter = props.on_update.clone();
        Callback::from(move |config: PidOutputLimit<f64>| {
            debug!("Output Change {:?} propagate", config );
            emitter.emit(Some(config));
        })
    };



    html! {
        <form  class="flex flex-row m-2 rounded border p-2 border-gray-400 dark:border-gray-600">
            <div class="flex flex-col w-40">
                <label for="open_loop_label" class="block mb-2 text-sm font-medium text-gray-300 dark:text-gray-700">
                    { "Limit Output" }
                </label>
                <div id="open_loop_label">
                    <label class="relative inline-flex items-center cursor-pointer">
                        <span class="ms-3 p-2 text-sm font-medium text-gray-900 dark:text-gray-300"> {"No"}</span>
                        <input type="checkbox" checked={*is_output_limited} onchange={output_limit_change} class="sr-only peer"/>
                        <div class="relative w-11 h-6 bg-gray-200 rounded-full peer peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600"></div>
                        <span class="ms-3 p-2 text-sm font-medium text-gray-900 dark:text-gray-300"> {"Yes"}</span>
                    </label>
                </div>
            </div>

           if *is_output_limited {
                <PidControllerLimitedOutputDialog config={
                    match props.config.clone() {
                        Some(c) => c,
                        None => PidOutputLimit::default(),
                    }
                } on_update={ output_change } />
           }
        </form>
    }
}


#[derive(Properties, PartialEq)]
pub struct PidControllerOutputDialogProps {
    pub config: PidOutputLimit<f64>,
    pub on_update: Callback<PidOutputLimit<f64>>,
}

#[function_component(PidControllerLimitedOutputDialog)]
pub fn pid_controller_limited_output_dialog(props: &PidControllerOutputDialogProps) -> Html {

    fn always_valid(_s: String) -> bool {
        true
    }

    let output_config = props.config.clone();
    debug!("PidControllerLimitedOutputDialog - Entry: {:?}", output_config);

    let output_min_ref = use_node_ref();
    let output_min_handle =  use_state(|| output_config.minimum().to_string());
    let output_min_valid_handle = use_state(|| true);

    let output_max_ref = use_node_ref();
    let output_max_handle =  use_state(|| output_config.maximum().to_string());
    let output_max_valid_handle = use_state(|| true);

    let anti_windup = use_state(|| output_config.anti_windup.clone());

    let windup_change = {
        let anti_windup = anti_windup.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            debug!("Anti-Windup {}", input.checked(), );
            anti_windup.set(input.checked());

        })
    };

    let updated = output_config.clone()
        .range(
            (*output_min_handle).parse::<f64>().unwrap_or_default(),
            (*output_max_handle).parse::<f64>().unwrap_or_default(),
        )
        .anti_windup(*anti_windup)
        ;
    if updated != output_config {
        props.on_update.emit(updated);
    }


    html! {
        <div class="flex flex-row">

            <Input
                r#type="number"
                name="output_min"
                r#ref={output_min_ref}
                handle={output_min_handle}
                valid_handle={output_min_valid_handle}
                validate_function={always_valid}

                label="Minimum Output"
                required={true}
                error_message="Must be a not-negative number"
                class="form-field w-32 pl-2 pr-2"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="error-text"
            />

            <Input
                r#type="number"
                name="output_max"
                r#ref={output_max_ref}
                handle={output_max_handle}
                valid_handle={output_max_valid_handle}
                validate_function={always_valid}

                label="Maximum Output"
                required={true}
                error_message="Must be a not negative number"
                class="form-field w-32 pl-2 pr-2"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="error-text"
            />

            <div class=" w-48">
                <label for="open_loop_label" class="block mb-2 text-sm font-medium text-gray-300 dark:text-gray-700">
                    { "Integral Adjustment" }
                </label>
                <div id="open_loop_label">
                    <label class="relative inline-flex items-center cursor-pointer">
                        <input type="checkbox" checked={*anti_windup} onchange={windup_change} class="sr-only peer"/>
                        <div class="relative w-11 h-6 bg-gray-200 rounded-full peer peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600"></div>
                        <span class="ms-3 p-2 text-sm font-medium text-gray-900 dark:text-gray-300"> {"Anti-Windup"}</span>
                    </label>
                </div>
            </div>

        </div>
    }
}