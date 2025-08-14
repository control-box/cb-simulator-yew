use accordion_rs::yew::{Accordion, Item, List};
use accordion_rs::Size;
use log::info;
use std::vec::Vec;
use yew::prelude::*;

use input_rs::yew::Input;

use control_box::plant::BoxedTransferTimeDomain;

use crate::components::plant::named_element::NamedElement;
use crate::components::plant::named_element_dialog::NamedElementDialog;
use crate::components::plant::element_select::*;

#[derive(Properties, PartialEq)]
pub struct AccordeonElementsProps {
    pub elements: UseStateHandle<Vec<NamedElement<f64>>>,
}

#[function_component(AccordeonElements)]
pub fn accordeon_elements(props: &AccordeonElementsProps) -> Html {
    let expand = use_state(|| true);

    let elements_handle = props.elements.clone();

    // State to hold the new element to add
    let new_handle = use_state_eq(|| {
        NamedElement::<f64>::default().set_name(format!("PT1-{}", props.elements.len() + 1))
    });
    fn always_valid(_s: String) -> bool {
        true
    }

    let name_ref = use_node_ref();
    let name_handle = use_state_eq(|| (*new_handle).name.clone());
    let name_valid_handle = use_state(|| true);

    let on_add = {
        let elements_handle = elements_handle.clone();
        let new_handle = new_handle.clone();
        let name_handle = name_handle.clone();

        Callback::from(move |_| {
            let mut elements = (*elements_handle).clone();
            let new = (*new_handle).clone();
            info!("Add new element: {}", new);
            elements.push(new.clone());
            let new_name = format!("{}-{}", new.element.short_type_name(), elements.len() + 1);
            elements_handle.set(elements);
            name_handle.set(new_name.clone());

            let new = new.set_name(new_name);
            info!("Element name update after ADD: {}", new.name);
            new_handle.set(new);
        })
    };

    let on_remove = {
        let elements_handle = elements_handle.clone();
        Callback::from(move |element_index: usize| {
            let mut elements = (*elements_handle).clone();
            if element_index < elements.len() {
                elements.remove(element_index);
                elements_handle.set(elements);
            }
        })
    };

    let on_update = {
        let elements_handle = elements_handle.clone();
        Callback::from(
            move |(element_index, element): (usize, NamedElement<f64>)| {
                info!("on_update: at {:?} value: {}", element_index, element);
                let mut elements = (*elements_handle).clone();
                if element_index < elements.len() {
                    let _ = std::mem::replace(&mut elements[element_index], element);
                    elements_handle.set(elements);
                }
            },
        )
    };

    let elements = (*elements_handle)
        .iter()
        .enumerate()
        .map(|(idx, element)| {
            let on_remove = {
                let on_remove = on_remove.clone();
                Callback::from(move |_| on_remove.emit(idx))
            };

            let on_update = {
                let on_update = on_update.clone();
                Callback::from(move |s| on_update.emit((idx, s)))
            };

            info!("IN-LIST-MAP Index: {} Element: {}", idx, element);

            html! {
                <Item class="flex flex-row">
                    <div class="flex flex-row items-center justify-between">
                        <button onclick={on_remove}
                            class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                            aria-label="Remove Element"
                        >
                            <span class="fa-solid fa-minus"></span>
                        </button>
                    </div>
                    <NamedElementDialog element={element.clone()} on_update={on_update} />
                </Item>
            }
        })
        .collect::<Html>();

    let on_element_type_change: Callback<BoxedTransferTimeDomain<f64>> = {
        let new_handle = new_handle.clone();
        let name_handle = name_handle.clone();
        let elements_handle = elements_handle.clone();

        Callback::from(move |element: BoxedTransferTimeDomain<f64>| {
            let elements_len = (*elements_handle).len();
            let type_name = format!("{}-{}", element.short_type_name(), elements_len + 1);
            let new_named_element = (*new_handle).clone().set_element(element);
            info!("Element_type_change: {}", new_named_element);
            new_handle.set(new_named_element);
            name_handle.set(type_name);
        })
    };

    let name = (*name_handle).parse::<String>().unwrap_or_default();
    let new_value = (*new_handle).clone().set_name(name.clone());
    new_handle.set(new_value);

    html! {
        <Accordion
            expand={expand}
            expanded={html! { "Simulated Elements/Plants/Processes" }}
            collapsed={html! {<>
                 { "Set Simulated Elements/Plants/Processes" }
            </>}}
            size={Size::Custom("auto")}
            class=" p-4 rounded border border-gray-400 dark:border-gray-600"
            expanded_class=" bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
        >
            <List>
                { elements }
                <Item class="flex flex-row content-start">
                    <div class="flex flex-row items-center justify-between">
                        <button onclick={on_add}
                            class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                            aria-label="Add an element"
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

                                label="Element Name"
                                required={true}
                                error_message="Must be a word"
                                class="form-field w-64"
                                label_class="block text-sm mb-2 text-gray-300 dark:text-gray-700"
                                input_class="w-full p-2 border border-gray-400 dark:border-gray-600 rounded"
                                error_class="text-red-800"
                            />
                        </form>
                        <ElementSelection onchange={on_element_type_change} />
                    </div>

                </Item>
            </List>
        </Accordion>

    }
}
