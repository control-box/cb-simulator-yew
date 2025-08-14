use yew::prelude::*;

use crate::components::plant::registry::list_factories;
use crate::components::plant::named_element::NamedElement;
#[derive(Properties, PartialEq)]
pub struct NamedElementDialogProps {
    pub element: NamedElement<f64>,
    /// The state handle for managing the value of the input.
    pub on_update: Callback<NamedElement<f64>>,
}

#[function_component(NamedElementDialog)]
pub fn element_dialog(props: &NamedElementDialogProps) -> Html {
    let updated = props.element.clone();
    let name = updated.name.clone();

    let on_update = {
        let emitter = props.on_update.clone();
        let updated = updated.clone();
        Callback::from(move |element| {
            let updated = updated.clone().set_element(element);
            emitter.emit(updated);
        })
    };

    let element = props.element.element.clone();
    html! {
        <div class="p-4">
        <div class="flex content-start flex-row rounded border p-2 border-gray-400 dark:border-gray-600">
            <div class="flex flex-col w-64">
                <label class="block text-sm text-gray-300 dark:text-gray-700 mb-2 form-field" for="element_name"> { "Element Name" } </label>
                <div id="element_name" class=" text-lg font-bold w-64">
                    { name}
                </div>
            </div>
            {
                // statically it would be:
                // match element.clone().short_type_name() {
                //     "Step" => html! { <StepFunctionDialog element={element.clone()} on_update={ on_update }/> },
                //     "Impulse" => html! { <ImpulseFunctionDialog element={element.clone()} on_update={ on_update } /> },
                //     _ => html! { format!("{}", element.clone()) }
                // }
                list_factories()
                    .into_iter()
                    .map(|factory|factory().dialog(element.clone(), on_update.clone()))
                    .collect::<Html>()
            }
            </div>
        </div>
    }
}
