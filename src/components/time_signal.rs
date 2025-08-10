use accordion_rs::yew::{Accordion, Item, List};
use accordion_rs::Size;
use log::info;
use std::vec::Vec;
use yew::prelude::*;

use input_rs::yew::Input;

use control_box::signal::*;

use crate::components::named_time_signal_dialog::NamedTimeSignalDialog;
use crate::components::time_signal_select::*;

#[derive(Properties, PartialEq)]
pub struct AccordeonTimeSignalsProps {
    pub signals: UseStateHandle<Vec<NamedTimeSignal<f64>>>,
}

#[function_component(AccordeonTimeSignals)]
pub fn accordeon_time_signals(props: &AccordeonTimeSignalsProps) -> Html {
    let expand = use_state(|| true);

    let signals_handle = props.signals.clone();

    // State to hold the new signal to add
    let new_handle = use_state_eq(|| {
        NamedTimeSignal::<f64>::default().set_name(format!("Signal-{}", props.signals.len() + 1))
    });
    fn always_valid(_s: String) -> bool {
        true
    }

    let name_ref = use_node_ref();
    let name_handle = use_state_eq(|| (*new_handle).name.clone());
    let name_valid_handle = use_state(|| true);

    let on_add = {
        let signals_handle = signals_handle.clone();
        let new_handle = new_handle.clone();
        let name_handle = name_handle.clone();

        Callback::from(move |_| {
            let mut signals = (*signals_handle).clone();
            let new = (*new_handle).clone();
            info!("Add new signal: {}", new);
            signals.push(new.clone());
            let new_name = format!("Signal-{}", signals.len() + 1);
            signals_handle.set(signals);
            name_handle.set(new_name.clone());

            let new = new.set_name(new_name);
            info!("Signal_name update after ADD: {}", new.name);
            new_handle.set(new);
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
                info!("on_update: at {:?} value: {}", signal_index, signal);
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

            info!("IN-LIST-MAP Index: {} Signal: {}", idx, signal);

            html! {
                <Item class="flex flex-row">
                    <div class="flex flex-row items-center justify-between">
                        <button onclick={on_remove}
                            class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                            aria-label="Remove Signal"
                        >
                            <span class="fa-solid fa-minus"></span>
                        </button>
                    </div>
                    <NamedTimeSignalDialog named_time_signal={signal.clone()} on_update={on_update} />
                </Item>
            }
        })
        .collect::<Html>();

    let on_signal_type_change: Callback<BoxedTimeSignal<f64>> = {
        let new_handle = new_handle.clone();

        Callback::from(move |signal: BoxedTimeSignal<f64>| {
            let new_named_signal = (*new_handle).clone().set_signal(signal);
            info!("Signal_type_change: {}", new_named_signal);
            new_handle.set(new_named_signal);
        })
    };

    let name = (*name_handle).parse::<String>().unwrap_or_default();
    let new_value = (*new_handle).clone().set_name(name.clone());
    new_handle.set(new_value);

    html! {
        <Accordion
            expand={expand}
            expanded={html! { " Times Signals" }}
            collapsed={html! {<>
                 { "Time Signals" }
            </>}}
            size={Size::Custom("auto")}
            class=" p-4 rounded border border-gray-400 dark:border-gray-600"
            expanded_class=" bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
        >
            <List>
                { signals }
                <Item class="flex flex-row content-start">
                    <div class="flex flex-row items-center justify-between">
                        <button onclick={on_add}
                            class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                            aria-label="Add a signal"
                        >
                            <span class="fa-solid fa-plus"></span>
                        </button>
                    </div>
                    <div class="flex flex-row p-4">
                        <form  class="flex flex-row pr-4">
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
                                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                                error_class="text-red-800"
                            />
                        </form>
                        <TimeSignalSelection onchange={on_signal_type_change} />
                    </div>

                </Item>
            </List>
        </Accordion>

    }
}
