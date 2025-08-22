use input_rs::yew::Input;
use yew::prelude::*;
use log::debug;


#[derive(Properties, PartialEq)]
pub struct PidControllerOptionalDeadBandDialogProps {
    pub config: Option<f64>,
    pub on_update: Callback<Option<f64>>,
}


#[function_component(PidControllerDeadBandDialog)]
pub fn pid_controller_dead_band_dialog(props: &PidControllerOptionalDeadBandDialogProps) -> Html {

    debug!("PidControllerDeadBandDialog - Entry: {:?}", props.config);

    let is_dead_band_set = use_state( || props.config.is_some());

    let dead_band_change = {
        let is_dead_band_set = is_dead_band_set.clone();
        let emitter = props.on_update.clone();
        Callback::from(move |e: Event| {

            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            debug!("DeadBand Tolerance {} Checkbox {}", *is_dead_band_set, input.checked(), );

            is_dead_band_set.set(input.checked());
            if !input.checked() { emitter.emit(None) }
        })
    };

    let input_change = {
        let emitter = props.on_update.clone();
        Callback::from(move |config: f64| {
            debug!("DeadBand Change {:?} propagate", config );
            emitter.emit(Some(config));
        })
    };



    html! {
        <form  class="flex flex-row m-2 rounded border p-2 border-gray-400 dark:border-gray-600">

            <div class="flex flex-col w-40">
                <label for="open_loop_label" class="block mb-2 text-sm font-medium text-gray-300 dark:text-gray-700">
                    { "Dead Band" }
                </label>
                <div id="open_loop_label">
                    <label class="relative inline-flex items-center cursor-pointer">
                        <span class="ms-3 p-2 text-sm font-medium text-gray-900 dark:text-gray-300"> {"No"}</span>
                        <input type="checkbox" checked={*is_dead_band_set} onchange={dead_band_change} class="sr-only peer"/>
                        <div class="relative w-11 h-6 bg-gray-200 rounded-full peer peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600"></div>
                        <span class="ms-3 p-2 text-sm font-medium text-gray-900 dark:text-gray-300"> {"Yes"}</span>
                    </label>
                </div>
            </div>

           if *is_dead_band_set {
                <PidControllerDeadBandToleranceDialog config={
                    match props.config.clone() {
                        Some(c) => c,
                        None => 1.0_f64,
                    }
                } on_update={ input_change } />
           }
        </form>
    }
}


#[derive(Properties, PartialEq)]
pub struct PidControllerDeadBandToleranceDialogProps {
    pub config: f64,
    pub on_update: Callback<f64>,
}

#[function_component(PidControllerDeadBandToleranceDialog)]
pub fn pid_controller_dead_band_dialog(props: &PidControllerDeadBandToleranceDialogProps) -> Html {

    fn positive_valid(s: String) -> bool {
        match s.parse::<f64>() {
            Ok(value) => value > 0.0 ,
            Err(_) => false,
        }
    }

    let dead_band_config = props.config.clone();
    debug!("PidControllerDeadBandDialog - Entry: {:?}", dead_band_config);

    let dead_band_ref = use_node_ref();
    let dead_band_handle =  use_state(|| dead_band_config.to_string());
    let dead_band_valid_handle = use_state(|| true);

    let updated = (*dead_band_handle).parse::<f64>().unwrap_or_default();
    if updated != props.config {
        props.on_update.emit(updated);
    }

    html! {
        <div class="flex flex-col w-32">
            <Input
                r#type="number"
                name="dead_band"
                r#ref={dead_band_ref}
                handle={dead_band_handle}
                valid_handle={dead_band_valid_handle}
                validate_function={positive_valid}

                label="Tolerance"
                required={true}
                error_message="Must be a positive number"
                class="form-field w-32 pl-2 pr-2"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="text-red-800 dark:text-red-200"
            />
        </div>
    }
}