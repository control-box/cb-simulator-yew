use input_rs::yew::Input;
use log::info;
use yew::prelude::*;

use crate::components::plant::registry::{register_element, YewElement};
use crate::components::plant::BoxedElementDialogProps;
use control_box::plant::{pt1::PT1, DynTransferTimeDomain, TypeIdentifier};

pub struct YewStep {
    element: PT1<f64>,
}

impl YewElement for YewStep {
    fn dialog(
        &self,
        element: Box<dyn DynTransferTimeDomain<f64>>,
        on_update: Callback<Box<dyn DynTransferTimeDomain<f64>>>,
    ) -> Html {
        if self.element().short_type_name() == element.short_type_name() {
            html! { <PT1Dialog element={element} on_update={ on_update }/> }
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
fn yew_pt1_factory() -> Box<dyn YewElement + Sync> {
    Box::new(YewStep {
        element: PT1::<f64>::default(),
    })
}

pub fn register() {
    info!("Registering YewPT1");
    register_element(yew_pt1_factory);
}

#[function_component(PT1Dialog)]
pub fn pt1_element_dialog(props: &BoxedElementDialogProps) -> Html {
    // Runtime reflection (downcasting to concrete type)
    // Variable assignment must be done outside the html! macro
    let updated = if let Some(pt1) = props.element.clone().as_any().downcast_ref::<PT1<f64>>() {
        pt1.clone()
    } else {
        PT1::<f64>::default()
    };

    fn always_valid(_s: String) -> bool {
        true
    }

    let kp_ref = use_node_ref();
    let kp_handle = use_state(|| updated.kp.to_string());
    let kp_valid_handle = use_state(|| true);

    let t1_time_ref = use_node_ref();
    let t1_time_handle = use_state(|| updated.t1_time.to_string());
    let t1_time_valid_handle = use_state(|| true);

    let sample_time_ref = use_node_ref();
    let sample_time_handle = use_state(|| updated.sample_time.to_string());
    let sample_time_valid_handle = use_state(|| true);

    let updated = PT1::<f64>::default()
        .set_sample_time((*sample_time_handle).parse::<f64>().unwrap_or_default())
        .set_t1_time((*t1_time_handle).parse::<f64>().unwrap_or_default())
        .set_kp((*kp_handle).parse::<f64>().unwrap_or_default());
    info!("PT1 updated: {}", updated);
    props.on_update.emit(Box::new(updated));

    html! {
        <div>
       <form  class="flex flex-row">
            <div class="flex flex-col w-64">
                <label class="block text-sm mb-2 form-field w-64 text-gray-300 dark:text-gray-700
                " for="pt1_element_label"> { "Element Type" } </label>
                <div id="pt1_element_label" class=" text-lg font-bold w-64"> { "PT1 Element"} </div>
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
                validate_function={always_valid}

                label="Time t1 [ms]"
                required={true}
                error_message="Must be a number"
                class="form-field w-64"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="error-text"
            />
            <Input
                r#type="number"
                name="sample_time"
                r#ref={sample_time_ref}
                handle={sample_time_handle}
                valid_handle={sample_time_valid_handle}
                validate_function={always_valid}

                label="Sample Time [ms]"
                required={true}
                error_message="Must greater than t1_time Time"
                class="form-field w-64"
                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                error_class="error-text"
            />
        </form>
        </div>
    }
}
