use yew::prelude::*;

use control_box::signal::*;

use crate::components::time_signal::impulse_fn::ImpulseFunctionDialog;
use crate::components::time_signal::step_fn::StepFunctionDialog;

#[derive(Properties, PartialEq)]
pub struct NamedTimeSignalDialogProps {
    pub time_signal: NamedTimeSignal<f64>,
    /// The state handle for managing the value of the input.
    pub on_update: Callback<NamedTimeSignal<f64>>,
}

#[function_component(NamedTimeSignalDialog)]
pub fn time_signal_dialog(props: &NamedTimeSignalDialogProps) -> Html {
    let updated = props.time_signal.clone();
    //let name = props.time_signal.name.clone();
    let name = updated.name.clone();

    let on_update = {
        let emitter = props.on_update.clone();
        let updated = updated.clone();
        Callback::from(move |signal| {
            let updated = updated.clone().set_signal(signal);
            emitter.emit(updated);
        })
    };

    let signal = props.time_signal.signal.clone();
    html! {
        <div class="p-4">
        <div class="flex content-start flex-row rounded border p-2 border-gray-400 dark:border-gray-600">
            <div class="flex flex-col w-64">
                <label class="block text-sm text-gray-300 dark:text-gray-700 mb-2 form-field" for="signal_name"> { "Signal Name" } </label>
                <div id="signal_name" class=" text-lg font-bold w-64">
                    { name}
                </div>
            </div>
            {

                match signal.clone().short_type_name() {
                    "Step" => html! { <StepFunctionDialog time_signal={signal.clone()} on_update={ on_update }/> },
                    "Impulse" => html! { <ImpulseFunctionDialog time_signal={signal.clone()} on_update={ on_update } /> },
                    _ => html! { format!("{}", signal.clone()) }
                }
            }
            </div>
        </div>
    }
}
