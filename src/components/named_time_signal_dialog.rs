use log::info;
use yew::prelude::*;

use control_box::signal::*;

use crate::components::impulse_fn::ImpulseFunctionDialog;
use crate::components::step_fn::StepFunctionDialog;

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

    if let Some(step) = props
        .named_time_signal
        .signal
        .clone()
        .as_any()
        .downcast_ref::<StepFunction<f64>>()
    {
        // Update only if trait object is a StepFunction
        info!("Step function found: {}", step);
        updated.signal = Box::new((*handle_step).clone());
    }
    if let Some(impulse) = props
        .named_time_signal
        .signal
        .clone()
        .as_any()
        .downcast_ref::<ImpulseFunction<f64>>()
    {
        // Update only if trait object is an ImpulseFunction
        info!("Impulse function found: {}", impulse);
        updated.signal = Box::new((*handle_impulse).clone());
        props.on_update.emit(updated.clone());
    }

    props.on_update.emit(updated);

    html! {
        <div class="p-4">
        <div class="flex content-start flex-row rounded border p-2 border-gray-400">
        <div class="flex flex-col w-64">
            <label class="block text-sm text-gray-300 mb-2 form-field" for="signal_name"> { "Signal Name" } </label>
            <div id="signal_name" class="text-gray-300 text-lg font-bold w-64">
                { name}
            </div>
         </div>
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
        </div>
    }
}
