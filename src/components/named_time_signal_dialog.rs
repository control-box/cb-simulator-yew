
use input_rs::yew::Input;
use log::info;
use yew::prelude::*;

use control_box::signal::*;

use crate::components::step_fn::StepFunctionDialog;
use crate::components::impulse_fn::ImpulseFunctionDialog;

#[derive(Properties, PartialEq)]
pub struct NamedTimeSignalDialogProps {
    pub named_time_signal: NamedTimeSignal<f64>,
    /// The state handle for managing the value of the input.
    pub on_update: Callback<NamedTimeSignal<f64>>,
}

#[function_component(NamedTimeSignalDialog)]
pub fn named_time_signal_dialog(props: &NamedTimeSignalDialogProps) -> Html {
    let mut updated = props.named_time_signal.clone();
    //let name = props.named_time_signal.name.clone();
    let name = updated.name.clone();


    info!("Named Time Signal called: {}", updated);
    fn always_valid(_s: String) -> bool {
        true
    }

    let name_ref = use_node_ref();
    let name_handle = use_state(|| updated.name.clone());
    let name_valid_handle = use_state(|| true);


    let signal_trait_object = props.named_time_signal.signal.clone();

    // Runtime reflection (downcasting to concrete type)
    // Variable assignment must be done outside the html! macro
    let step_fn = if let Some(step) = signal_trait_object
        .as_any()
        .downcast_ref::<StepFunction<f64>>()
    {
        step.clone()
    } else {
        StepFunction::<f64>::default()
    };
    let handle_step = { use_state(|| step_fn.clone()) };


    let impulse_fn = if let Some(impulse) = signal_trait_object
        .as_any()
        .downcast_ref::<ImpulseFunction<f64>>()
    {
        impulse.clone()
    } else {
        ImpulseFunction::<f64>::default()
    };
    let handle_impulse = { use_state(|| impulse_fn.clone()) };

    if let Some(step) = props.named_time_signal.signal.clone().as_any().downcast_ref::<StepFunction<f64>>() {
        // Update only if trait object is a StepFunction
        info!("Step function found: {}", step);
        updated.signal = Box::new((*handle_step).clone());
    }
    if let Some(impulse) = props.named_time_signal.signal.clone().as_any().downcast_ref::<ImpulseFunction<f64>>() {
        // Update only if trait object is an ImpulseFunction
        info!("Impulse function found: {}", impulse);
        updated.signal = Box::new((*handle_impulse).clone());
        props.on_update.emit(updated.clone());
    }

    updated.name = (*name_handle).parse::<String>().unwrap_or_default();
    props.on_update.emit(updated);

    html! {
        <div class="flex flex-row">
        { name}
       <form  class="flex flex-row">
            <Input
                r#type="text"
                name="name"
                r#ref={name_ref}
                handle={name_handle}
                valid_handle={name_valid_handle}
                validate_function={always_valid}

                label="Signal Name"
                required={true}
                error_message="Must be a word"
                class="form-field w-64"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="text-red-800"
            />
        </form>

        {
            if let Some(_) = signal_trait_object.as_any().downcast_ref::<StepFunction<f64>>() {
                html! { <StepFunctionDialog handle={handle_step} /> }
            } else {
                if let Some(_) = signal_trait_object.as_any().downcast_ref::<ImpulseFunction<f64>>() {
                   // html! { format!("{}", props.named_time_signal.signal.clone()) }
                   html! { <ImpulseFunctionDialog handle={handle_impulse} /> }
                } else {
                    html! { format!("{}", props.named_time_signal.signal.clone()) }
                }
            }
        }


        </div>
    }
}
