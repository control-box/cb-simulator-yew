use input_rs::yew::Input;
use log::info;
use yew::prelude::*;


use crate::components::plant::BoxedElementDialogProps;
use crate::plant::registry::{register_element, YewElement};
use cb_simulation_util::plant::{pt0::PT0, DynTransferTimeDomain, TypeIdentifier};

pub struct YewStep {
    element: PT0<f64>,
}

impl YewElement for YewStep {
    fn dialog(
        &self,
        element: Box<dyn DynTransferTimeDomain<f64>>,
        on_update: Callback<Box<dyn DynTransferTimeDomain<f64>>>,
        sample_time: f64,
    ) -> Html {
        if self.element().short_type_name() == element.short_type_name() {
            html! { <PT0Dialog element={element} on_update={ on_update } sample_time={ sample_time } /> }
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
fn yew_pt0_factory() -> Box<dyn YewElement + Sync> {
    Box::new(YewStep {
        element: PT0::<f64>::default(),
    })
}

pub fn register() {
    info!("Registering YewPT0");
    register_element(yew_pt0_factory);
}

#[function_component(PT0Dialog)]
pub fn pt0_element_dialog(props: &BoxedElementDialogProps) -> Html {
    // Runtime reflection (downcasting to concrete type)
    // Variable assignment must be done outside the html! macro
    let updated = if let Some(pt0) = props.element.clone().as_any().downcast_ref::<PT0<f64>>() {
        pt0.clone()
    } else {
        PT0::<f64>::default()
    };

    fn always_valid(_s: String) -> bool {
        true
    }

    fn positive_valid(s: String) -> bool {
        match s.parse::<f64>() {
            Ok(value) => value > 0.0 ,
            Err(_) => false,
        }
    }

    let kp_ref = use_node_ref();
    let kp_handle = use_state(|| updated.kp.to_string());
    let kp_valid_handle = use_state(|| true);

    let t0_time_ref = use_node_ref();
    let t0_time_handle = use_state(|| updated.t0_time.to_string());
    let t0_time_valid_handle = use_state(|| true);

    let updated = PT0::<f64>::default()
        .set_sample_time_or_default(props.sample_time.clone())
        .set_t0_time_or_default((*t0_time_handle).parse::<f64>().unwrap_or(1.0))
        .set_kp((*kp_handle).parse::<f64>().unwrap_or_default());
    info!("PT0 updated: {}", updated);
    props.on_update.emit(Box::new(updated));

    html! {
        <div>
       <form  class="flex flex-row">
            <div class="flex flex-col w-64">
                <label class="block text-sm mb-2 form-field w-64 text-gray-300 dark:text-gray-700
                " for="pt0_element_label"> { "Element Type" } </label>
                <div id="pt0_element_label" class=" text-lg font-bold w-64"> { "PT0 Element"} </div>
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
                name="t0_time"
                r#ref={t0_time_ref}
                handle={t0_time_handle}
                valid_handle={t0_time_valid_handle}
                validate_function={positive_valid}

                label="Time t0 [ms]"
                required={true}
                error_message="Must be a positive number"
                class="form-field w-64"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="error-text"
            />
        </form>
        </div>
    }
}
