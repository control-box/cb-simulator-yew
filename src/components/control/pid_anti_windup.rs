use input_rs::yew::Input;
use yew::prelude::*;
use log::debug;

use cb_controller::pid::PidAntiWindup;
use web_sys::HtmlSelectElement;


#[derive(Properties, PartialEq)]
pub struct PidControllerAntiWindupProps {
    pub config: PidAntiWindup<f64>,
    pub on_update: Callback<PidAntiWindup<f64>>,
}

#[function_component(PidControllerAntiWindup)]
pub fn pid_controller_limited_output_dialog(props: &PidControllerAntiWindupProps) -> Html {

    debug!("PidControllerAntiWindup - Entry config: {:?}", props.config);
    fn always_valid(_s: String) -> bool {
        true
    }

    let anti_windup_config = props.config.clone();
    debug!("PidControllerAntiWindup - Entry: {:?}", anti_windup_config);

    let beta_ref = use_node_ref();
    let beta_handle =  use_state(||
        if let PidAntiWindup::CompensatingIntegration(beta) = anti_windup_config.clone() {
            beta.to_string()
        } else {
            "0.0".to_string()
        });
    let beta_valid_handle = use_state(|| true);

    let anti_windup = use_state(|| anti_windup_config.clone());

    let windup_change = {
        let anti_windup = anti_windup.clone();
        let beta_handle = beta_handle.clone();        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlSelectElement>();
            if let Some(select) = target {
                match select.value().as_str() {
                    "CompensatingIntegration" => {
                        let beta = (*beta_handle).parse::<f64>().unwrap_or(0.0);
                        anti_windup.set(PidAntiWindup::CompensatingIntegration(beta));
                        debug!("Compensating Integration at beta={beta}");
                    }
                    "IntegralReset" => {
                        anti_windup.set(PidAntiWindup::IntegralReset);
                        debug!("Reset Integration");
                    }
                    "NoAntiWindup" => {
                        anti_windup.set(PidAntiWindup::NoAntiWindup);
                        debug!("No Anti-Windup");
                    }
                    _ => {}
                }
            }
        })
    };


    let mut updated = (*anti_windup).clone();
    if let PidAntiWindup::CompensatingIntegration(beta) = updated {
        updated = PidAntiWindup::CompensatingIntegration(
                    (*beta_handle).parse::<f64>().unwrap_or(beta)
                );
    }

    if updated != anti_windup_config {
        debug!("PidControllerAntiWindup - Updated: {:?}", updated);
        props.on_update.emit(updated);
    }


    html! {
        <div class="flex flex-row">

            <div class=" w-48">
                <label for="anti_windup_label" class="block mb-2 text-sm font-medium text-gray-300 dark:text-gray-700"> { "Anti Windup" } </label>
                <select name={"Anti-Windup"} onchange={windup_change}
                        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        id="anti_windup_label">

                    <option value="CompensatingIntegration"
                        selected ={  matches!(anti_windup_config.clone(), PidAntiWindup::CompensatingIntegration(_))}>
                        { "Backward Calculation" }
                    </option>
                    <option value="IntegralReset"
                        selected = {matches!(anti_windup_config.clone(), PidAntiWindup::IntegralReset)}>
                        { "Integral Reset" }
                    </option>
                        <option value="NoAntiWindup"
                        selected = {matches!(anti_windup_config, PidAntiWindup::NoAntiWindup)}>
                        { "None" }
                    </option>
                </select>
            </div>

            { if matches!(*anti_windup, PidAntiWindup::CompensatingIntegration(_)) {
                html! {
                    <Input
                        r#type="number"
                        name="beta"
                        r#ref={beta_ref}
                        handle={beta_handle}
                        valid_handle={beta_valid_handle}
                        validate_function={always_valid}

                        label="Beta"
                        required={true}
                        error_message="Must be a not-negative number"
                        class="form-field w-32 pl-2 pr-2"
                        label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                        input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                        error_class="error-text"
                    />
                }
            } else {
                html! {}
            }}

        </div>
    }
}