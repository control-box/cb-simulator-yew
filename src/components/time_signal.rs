use accordion_rs::yew::{Accordion, Item, List};
use accordion_rs::Size;
use input_rs::yew::Input;
use log::info;
use std::vec::Vec;
use yew::prelude::*;

use control_box::signal::*;

use crate::components::step_fn::StepFunctionDialog;

use crate::components::time_signal_select::*;

#[derive(Properties, PartialEq)]
pub struct AccordeonTimeSignalsProps {
    pub signals: UseStateHandle<Vec<NamedTimeSignal<f64>>>,
}

#[function_component(AccordeonTimeSignals)]
pub fn accordeon_time_signals(props: &AccordeonTimeSignalsProps) -> Html {
    let expand = use_state(|| true);

    let signals_handle = props.signals.clone();

    let on_add = {
        let signals_handle = signals_handle.clone();
        Callback::from(move |new: NamedTimeSignal<f64>| {
            let mut signals = (*signals_handle).clone();
            signals.push(new);
            signals_handle.set(signals);
        })
    };

    let on_remove = {
        let signals_handle = signals_handle.clone();
        Callback::from(move |signal_index: usize| {
            let mut signals = (*signals_handle).clone();
            if signal_index < signals.len() {
                signals.remove(signal_index);
                signals_handle.set(signals);
            }
        })
    };

    let on_update = {
        let signals_handle = signals_handle.clone();
        Callback::from(
            move |(signal_index, signal): (usize, NamedTimeSignal<f64>)| {
                info!(
                    "on_update called for index {:?} new value: {:?}",
                    signal_index, signal.name
                );
                let mut signals = (*signals_handle).clone();
                if signal_index < signals.len() {
                    let _ = std::mem::replace(&mut signals[signal_index], signal);
                    signals_handle.set(signals);
                }
            },
        )
    };

    let signals = (*signals_handle)
        .iter()
        .enumerate()
        .map(|(idx, signal)| {
            let on_remove = {
                let on_remove = on_remove.clone();
                Callback::from(move |_| on_remove.emit(idx))
            };

            let on_update = {
                let on_update = on_update.clone();
                Callback::from(move |s| on_update.emit((idx, s)))
            };

            html! {
                <Item class="flex flex-row">
                    <button onclick={on_remove}
                        class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                        aria-label="Remove Signal"
                    >
                        <span class="fa-solid fa-minus"></span> { "Remove"}
                    </button>
                    <NameTimeSignalDialog named_time_signal={signal.clone()} on_update={on_update} />
                </Item>
            }
        })
        .collect::<Html>();

    let add_default = use_state(|| NamedTimeSignal::<f64>::default());

    let on_signal_type_change: Callback<BoxedTimeSignal<f64>> = {
        let add_default = add_default.clone();

        Callback::from(move |signal: BoxedTimeSignal<f64>| {
            info!("on_signal_type_change called for signal: {:?}", signal);
            let new_signal = NamedTimeSignal::<f64> {
                name: "New Signal".to_string(),
                signal,
            };
            add_default.set(new_signal.clone());
        })
    };

    html! {
        <Accordion
            expand={expand}
            expanded={html! { " Times Signals" }}
            collapsed={html! {<>
                 { "Time Signals:" }
            </>}}
            size={Size::Custom("auto")}
            class="my-custom-class bg-gray-800 p-4 rounded border border-gray-400"
            expanded_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
        >
            <List>
                { signals }
                <Item class="flex flex-row">
                  <button onclick={Callback::from(move |_| on_add.emit((*add_default).clone()))}
                    class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Add a signal"
                  >
                    <span class="fa-solid fa-plus"></span> { "Add"}
                  </button>
                  <TimeSignalSelection onchange={on_signal_type_change} />

                </Item>
            </List>
        </Accordion>

    }
}

#[derive(Properties, PartialEq)]
pub struct NameTimeSignalDialogProps {
    #[prop_or_default]
    pub named_time_signal: NamedTimeSignal<f64>,
    /// The state handle for managing the value of the input.
    pub on_update: Callback<NamedTimeSignal<f64>>,
}

#[function_component(NameTimeSignalDialog)]
pub fn named_time_signal_dialog(props: &NameTimeSignalDialogProps) -> Html {
    let mut updated = props.named_time_signal.clone();

    fn always_valid(_s: String) -> bool {
        true
    }

    let name_ref = use_node_ref();
    let name_handle = use_state(|| updated.name.clone());
    let name_valid_handle = use_state(|| true);

    updated.name = (*name_handle).parse::<String>().unwrap_or_default();
    info!("Updated name: {}", updated.name);
    // props.on_update.emit(updated);

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
        .downcast_ref::<ImpulsFunction<f64>>()
    {
        impulse.clone()
    } else {
        ImpulsFunction::<f64>::default()
    };
    let handle_impulse = { use_state(|| impulse_fn.clone()) };

    html! {
        <div class="flex flex-row">
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
                if let Some(_) = signal_trait_object.as_any().downcast_ref::<ImpulsFunction<f64>>() {
                    html! { format!("{}", props.named_time_signal.signal.clone()) }
                   // html! { <ImpulseFunctionDialog handle={handle_impulse} /> }
                } else {
                    html! { format!("{}", props.named_time_signal.signal.clone()) }
                }
            }
        }


        </div>
    }
}
