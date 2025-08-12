use strum::{EnumIter, IntoEnumIterator};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use log::info;

#[derive(Debug, Default)]
pub struct YewStepFn {}

#[derive(Debug, Default)]
pub struct YewImpulseFn {}

#[derive(Debug, EnumIter)]
pub enum TimeSignalTypes {
    Step(YewStepFn),
    Impulse(YewImpulseFn),
}

use control_box::signal::{BoxedTimeSignal, ImpulseFunction, StepFunction};

#[derive(Properties, PartialEq)]
pub struct TimeSignalSelectProps {
    pub onchange: Callback<BoxedTimeSignal<f64>>,
}

#[function_component(TimeSignalSelection)]
pub fn time_signal_selection(props: &TimeSignalSelectProps) -> Html {
    // Collect options for the select dropdown
    let time_signal_types = TimeSignalTypes::iter()
        .enumerate()
        .map(|(index, signal_type)| {
            html! {
                <option  value={index.to_string()}
                    selected={index == 0} // if the list get changed always the first element is selected
                >
                    { format!("{:?}", signal_type) }
                </option>
            }
        })
        .collect::<Vec<Html>>();

    let on_change = {
        let emitter = props.onchange.clone();

        Callback::from(move |event: Event| {
            let target = event.target_dyn_into::<HtmlSelectElement>();
            if let Some(select) = target {
                let selected = select.value().parse::<usize>().unwrap_or(0);
                for (pos, element) in TimeSignalTypes::iter().enumerate() {
                    if pos == selected {
                        match element {
                            TimeSignalTypes::Step(_) => {
                                info!("Selected Step Function");
                                emitter.emit(Box::new(StepFunction::<f64>::default()));
                            }
                            TimeSignalTypes::Impulse(_) => {
                                info!("Selected Impulse Function");
                                emitter.emit(Box::new(ImpulseFunction::<f64>::default()));
                            }
                        }
                        break;
                    }
                }
            }
        })
    };

    html! {
        <div class=" mx-auto">
        <label for="signal_type_label" class="block mb-2 text-sm font-medium text-gray-300 dark:text-gray-700"> { "Select Signal Type" } </label>
        <select name={"signal"} onchange={ on_change}
            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
         id="signal_type_label">
            { time_signal_types }
        </select>
        </div>
    }
}
