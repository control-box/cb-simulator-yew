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

use control_box::signal::{
    BoxedTimeSignal, ImpulsFunction, StepFunction, SuperPosition, TimeSignalSuperTrait,
};

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
                <option value={index.to_string()}
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
                                emitter.emit(Box::new(ImpulsFunction::<f64>::default()));
                            }
                        }
                        break;
                    }
                }
            }
        })
    };

    html! {
        <select name={"signal"} onchange={ on_change}
            class="block text-sm text-gray-300 mb-2 w-64"
         id="signal_type_label">
            { time_signal_types }
        </select>
    }
}
