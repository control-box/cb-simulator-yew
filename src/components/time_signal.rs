use accordion_rs::yew::{Accordion, Item, List};
use accordion_rs::Size;
use log::info;
use std::vec::Vec;
use yew::prelude::*;

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
    let new_signal_to_add = use_state(||
        NamedTimeSignal::<f64>::default().set_name(format!("Signal-{}", props.signals.len() + 1 )));

    let on_add = {
        let signals_handle = signals_handle.clone();
        let new_signal_to_add = new_signal_to_add.clone();

        Callback::from(move |_| {
            let mut signals = (*signals_handle).clone();
            let new_name = format!("Signal-{}", signals.len() + 1 );
            let new = (*new_signal_to_add).clone().set_name(new_name);
            info!("Add new signal: {}", new);
            signals.push(new);
            info!("Last name: {}", signals.last().map_or("None".to_string(), |s| s.name.clone()));
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
                    "On_update called for index {:?} new value: {}",
                    signal_index, signal
                );
                let mut signals = (*signals_handle).clone();
                if signal_index < signals.len() {
                    info!("Replace!!!!");
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
                    <div class="flex flex-row items-center justify-between">
                    <button onclick={on_remove}
                        class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-24 h-12 rounded-lg text-xl leading-12"
                        aria-label="Remove Signal"
                    >
                        <span class="fa-solid fa-minus"></span> { "Remove"}
                    </button>
                                      </div>
                    <NamedTimeSignalDialog named_time_signal={signal.clone()} on_update={on_update} />
                </Item>
            }
        })
        .collect::<Html>();


    let on_signal_type_change: Callback<BoxedTimeSignal<f64>> = {
        let new_signal_to_add = new_signal_to_add.clone();

        Callback::from(move |signal: BoxedTimeSignal<f64>| {
            info!("on_signal_type_change called for signal: {:?}", signal);
            new_signal_to_add.set(NamedTimeSignal::<f64>::default().set_signal(signal));
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
                <div class="flex flex-row items-center justify-between">
                  <button onclick={on_add}
                    class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Add a signal"
                  >
                    <span class="fa-solid fa-plus"></span> { "Add"}
                  </button>
                  </div>
                  <TimeSignalSelection onchange={on_signal_type_change} />

                </Item>
            </List>
        </Accordion>

    }
}
