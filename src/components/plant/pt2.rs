use input_rs::yew::Input;
use log::info;
use yew::prelude::*;

use crate::components::plant::BoxedElementDialogProps;
use crate::plant::registry::{register_element, YewElement};
use cb_simulation_util::plant::{pt2::PT2, DynTransferTimeDomain, TypeIdentifier};

pub struct YewStep {
    element: PT2<f64>,
}

impl YewElement for YewStep {
    fn dialog(
        &self,
        element: Box<dyn DynTransferTimeDomain<f64>>,
        on_update: Callback<Box<dyn DynTransferTimeDomain<f64>>>,
        sample_time: f64,
    ) -> Html {
        if self.element().short_type_name() == element.short_type_name() {
            html! { <PT2Dialog element={element} on_update={ on_update } sample_time={ sample_time } /> }
        } else {
            html! {}
        }
    }

    fn name(&self) -> &'static str {
        self.element.short_type_name()
    }

    fn render(&self) -> Html {
        html! { <> { self.element.short_type_name() } </> }
    }

    fn element(&self) -> Box<dyn DynTransferTimeDomain<f64> + Send + Sync> {
        Box::new(self.element.clone())
    }
}
fn yew_pt2_factory() -> Box<dyn YewElement + Sync> {
    Box::new(YewStep {
        element: PT2::<f64>::default(),
    })
}

pub fn register() {
    info!("Registering YewPT2");
    register_element(yew_pt2_factory);
}

#[function_component(PT2Dialog)]
pub fn pt2_element_dialog(props: &BoxedElementDialogProps) -> Html {
    // Runtime reflection (downcasting to concrete type)
    // Variable assignment must be done outside the html! macro
    let updated = if let Some(pt2) = props.element.clone().as_any().downcast_ref::<PT2<f64>>() {
        pt2.clone()
    } else {
        PT2::<f64>::default()
    };

    fn always_valid(_s: String) -> bool {
        true
    }

    fn positive_valid(s: String) -> bool {
        match s.parse::<f64>() {
            Ok(value) => value >= 0.0,
            Err(_) => false,
        }
    }

    // Validation closure for t1_time
    let t1_time_valid = {
        let sample_time = props.sample_time;
        Callback::from(move |s: String| {
            match s.parse::<f64>() {
                Ok(value) => value > sample_time,
                Err(_) => false,
            }
        })
    };

    let kp_ref = use_node_ref();
    let kp_handle = use_state(|| updated.kp.to_string());
    let kp_valid_handle = use_state(|| true);

    let t1_time_ref = use_node_ref();
    let t1_time_handle = use_state(|| (1.0 / updated.omega).to_string());
    let t1_time_valid_handle = use_state(|| true);

    let damping_ref = use_node_ref();
    let damping_handle = use_state(|| updated.damping.to_string());
    let damping_valid_handle = use_state(|| true);


    let updated = PT2::<f64>::default()
        .set_sample_time_or_default(props.sample_time.clone())
        .set_t1_time_or_default((*t1_time_handle).parse::<f64>().unwrap_or_default())
        .set_damping_or_default((*damping_handle).parse::<f64>().unwrap_or_default())
        .set_kp((*kp_handle).parse::<f64>().unwrap_or(1.0));
    info!("PT2 updated: {}", updated);
    props.on_update.emit(Box::new(updated));

    html! {
        <div>
       <form  class="flex flex-row">
            <div class="flex flex-col w-64">
                <label class="block text-sm mb-2 form-field w-64 text-gray-300 dark:text-gray-700
                " for="pt2_element_label"> { "Element Type" } </label>
                <div id="pt2_element_label" class=" text-lg font-bold w-64"> { "PT2 Element"} </div>
            </div>
            <Input
                r#type="number"
                name="kp"
                r#ref={kp_ref}
                handle={kp_handle}
                valid_handle={kp_valid_handle}
                validate_function={always_valid}

                label="Kp Amplification"
                required={true}
                error_message="Must be a number"
                class="form-field w-64"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="text-red-800"
            />
            <Input
                r#type="number"
                name="t1_time"
                r#ref={t1_time_ref}
                handle={t1_time_handle}
                valid_handle={t1_time_valid_handle}
                validate_function={t1_time_valid}

                label="Period time equivalent [ms] (1/omega)"
                required={true}
                error_message="Must be a number greater than sampling rate"
                class="form-field w-64"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="error-text"
            />
            <Input
                r#type="number"
                name="damping"
                r#ref={damping_ref}
                handle={damping_handle}
                valid_handle={damping_valid_handle}
                validate_function={positive_valid}

                label="Damping Factor"
                required={true}
                error_message="Must be a number"
                class="form-field w-64"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="error-text"
            />
        </form>
        </div>
    }
}
