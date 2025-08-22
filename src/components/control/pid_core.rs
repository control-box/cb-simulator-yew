use input_rs::yew::Input;
use yew::prelude::*;
use log::debug;

use cb_controller::pid::PidCoreBuilder;


#[derive(Properties, PartialEq)]
pub struct PidControllerDialogProps {
    pub builder: PidCoreBuilder<f64>,
    pub on_update: Callback<PidCoreBuilder<f64>>,
}


#[function_component(PidControllerDialog)]
pub fn pid_builder_dialog(props: &PidControllerDialogProps) -> Html {

    fn positive_valid(s: String) -> bool {
        match s.parse::<f64>() {
            Ok(value) => value > 0.0 ,
            Err(_) => false,
        }
    }

    fn not_negative_valid(s: String) -> bool {
        match s.parse::<f64>() {
            Ok(value) => value >= 0.0 ,
            Err(_) => false,
        }
    }

    debug!("PidCoreBuilder (Entry): {:?}", props.builder);
    let kp_value_ref = use_node_ref();
    let kp_value_handle = use_state(|| props.builder.kp.to_string());
    let kp_valid_handle = use_state(|| true);

    let is_time_mode = use_state(|| props.builder.is_time_parameterized());

    let ki_value_ref = use_node_ref();
    let ki_value_handle =  use_state(|| props.builder.get_ki().to_string());
    let ki_valid_handle = use_state(|| true);

    let reset_time_value_ref = use_node_ref();
    let reset_time_value_handle =  use_state(|| props.builder.get_reset_time().to_string());
    let reset_time_valid_handle = use_state(|| true);

    let kd_value_ref = use_node_ref();
    let kd_value_handle = use_state(|| props.builder.get_kd().to_string());
    let kd_valid_handle = use_state(|| true);

    let hold_time_value_ref = use_node_ref();
    let hold_time_value_handle = use_state(|| props.builder.get_hold_time().to_string());
    let hold_time_valid_handle = use_state(|| true);

    let time_mode_change = {
        let is_time_mode = is_time_mode.clone();
        Callback::from(move |e: Event| {

            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            debug!("Time mode is {}" , input.checked(), );

            is_time_mode.set(input.checked());
        })
    };

    let updated = props.builder.clone()  // use the props.builder here to prevent endless updates
        .kp((*kp_value_handle).parse::<f64>().unwrap_or_default());

    let updated = if *is_time_mode {
        let reset_time =(*reset_time_value_handle).parse::<f64>().unwrap_or_default();
        let hold_time =(*hold_time_value_handle).parse::<f64>().unwrap_or_default();
        debug!("Update reset time {:?} hold time {:?}", reset_time, hold_time);
        let updated = updated.reset_time(reset_time).hold_time(hold_time);
        // update the corresponding input values - ONLY if changed
        let field_ki = (*ki_value_handle).parse::<f64>().unwrap_or_default();
        let desired_ki = updated.get_ki();
        if field_ki != desired_ki {
            ki_value_handle.set(desired_ki.to_string());
        }
        let field_kd = (*kd_value_handle).parse::<f64>().unwrap_or_default();
        let desired_kd = updated.get_kd();
        if field_kd != desired_kd {
            kd_value_handle.set(desired_kd.to_string());
        }
        updated
    } else {
        let ki = (*ki_value_handle).parse::<f64>().unwrap_or_default();
        let kd = (*kd_value_handle).parse::<f64>().unwrap_or_default();
        debug!("Update ki {:?} kd {:?}", ki, kd);
        let updated = updated.ki(ki).kd(kd);

        // update the corresponding input values - ONLY if changed
        let field_reset_time = (*reset_time_value_handle).parse::<f64>().unwrap_or_default();
        let desired_reset_time = updated.get_reset_time();
        if field_reset_time != desired_reset_time {
            reset_time_value_handle.set(desired_reset_time.to_string());
        }
        let field_hold_time = (*hold_time_value_handle).parse::<f64>().unwrap_or_default();
        let desired_hold_time = updated.get_hold_time();
        if field_hold_time != desired_hold_time {
            hold_time_value_handle.set(desired_hold_time.to_string());
        }
        updated
    };

    if props.builder != updated {
        props.on_update.emit(updated);
    }

    html! {
       <form  class="flex flex-row m-2 rounded border p-2 border-gray-400 dark:border-gray-600">
            <Input
                r#type="number"
                name="kp_value"
                r#ref={kp_value_ref}
                handle={kp_value_handle}
                valid_handle={kp_valid_handle}
                validate_function={positive_valid}

                label="Proportional Kp"
                required={true}
                error_message="Must be a positive number"
                class="form-field w-32"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="text-red-800 dark:text-red-200"
            />

            <div class="flex flex-col w-48 pl-2 pr-2">
                <label for="open_loop_label" class="block mb-2 pl-4 text-sm font-medium text-gray-300 dark:text-gray-700">
                    { "Parameter Enter Mode" }
                </label>
                <div id="open_loop_label">
                    <label class="relative inline-flex items-center cursor-pointer">
                        <span class="ms-3 p-2 text-sm font-medium text-gray-900 dark:text-gray-300"> {"Factor"}</span>
                        <input type="checkbox" checked={*is_time_mode} onchange={time_mode_change} class="sr-only peer"/>
                        <div class="relative w-11 h-6 bg-gray-200 rounded-full peer peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600"></div>
                        <span class="ms-3 p-2 text-sm font-medium text-gray-900 dark:text-gray-300"> {"Time"}</span>
                    </label>
                </div>
            </div>

            <Input
                r#type="number"
                name="ki_value"
                r#ref={ki_value_ref}
                handle={ki_value_handle}
                valid_handle={ki_valid_handle}
                validate_function={not_negative_valid}
                disabled={*is_time_mode}

                label="Integral Ki"
                required={true}
                error_message="Must be a not-negative number"
                class="form-field w-32"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class={ if *is_time_mode { "w-full p-2"}
                    else { "w-full p-2 border border-gray-400 dark:border-gray-600 rounded" }}
                error_class="text-red-800 dark:text-red-200"
            />
            <Input
                r#type="number"
                name="reset_time_value"
                r#ref={reset_time_value_ref}
                handle={reset_time_value_handle}
                valid_handle={reset_time_valid_handle}
                validate_function={positive_valid}
                disabled={!*is_time_mode}

                label="Reset Time"
                required={true}
                error_message="Must be a positive number"
                class="form-field w-32"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class={ if *is_time_mode { "w-full p-2 border border-gray-400 dark:border-gray-600 rounded"}
                    else { "w-full p-2" }}
                error_class="text-red-800 dark:text-red-200"
            />

            <Input
                r#type="number"
                name="kd_value"
                r#ref={kd_value_ref}
                handle={kd_value_handle}
                valid_handle={kd_valid_handle}
                validate_function={not_negative_valid}
                disabled={*is_time_mode}

                label="Differential Kd"
                required={true}
                error_message="Must be a not-negative number"
                class="form-field w-32"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class={ if *is_time_mode { "w-full p-2"}
                    else { "w-full p-2 border border-gray-400 dark:border-gray-600 rounded" }}
                error_class="text-red-800 dark:text-red-200"
            />
            <Input
                r#type="number"
                name="hold_time_value"
                r#ref={hold_time_value_ref}
                handle={hold_time_value_handle}
                valid_handle={hold_time_valid_handle}
                validate_function={not_negative_valid}
                disabled={!*is_time_mode}

                label="Hold Time"
                required={true}
                error_message="Must be a not-negative number"
                class="form-field w-32"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class={ if *is_time_mode { "w-full p-2 border border-gray-400 dark:border-gray-600 rounded"}
                    else { "w-full p-2" }}
                error_class="text-red-800 dark:text-red-200"
            />

        </form>
    }
}
