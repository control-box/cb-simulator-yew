use crate::components::plant::registry::list_factories;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use log::info;

use control_box::plant::BoxedTransferTimeDomain;

#[derive(Properties, PartialEq)]
pub struct ElementSelectProps {
    pub onchange: Callback<BoxedTransferTimeDomain<f64>>,
}

#[function_component(ElementSelection)]
pub fn element_selection(props: &ElementSelectProps) -> Html {
    // Collect options for the select dropdown
    let element_types = list_factories()
        .into_iter()
        .enumerate()
        .map(|(index, factory)| {
            html! {
                <option  value={index.to_string()}
                    selected={index == 0} // if the list get changed always the first element is selected
                >
                    { factory().render() }
                </option>
            }
        })
        .collect::<Html>();

    let on_change = {
        let emitter = props.onchange.clone();

        Callback::from(move |event: Event| {
            let target = event.target_dyn_into::<HtmlSelectElement>();
            if let Some(select) = target {
                let selected = select.value().parse::<usize>().unwrap_or(0);
                for (pos, factory) in list_factories().into_iter().enumerate() {
                    if pos == selected {
                        info!("ADD: Selected element type: {}", factory().name());
                        emitter.emit(factory().element());
                        break;
                    }
                }
            }
        })
    };

    html! {
        <div class=" mx-auto">
        <label for="element_type_label" class="block mb-2 text-sm font-medium text-gray-300 dark:text-gray-700"> { "Select Element Type" } </label>
        <select name={"element"} onchange={ on_change}
            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
         id="element_type_label">
            { element_types }
        </select>
        </div>
    }
}
