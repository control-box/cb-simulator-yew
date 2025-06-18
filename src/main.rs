use yew::prelude::*;
use yew_accordion::{Accordion, AccordionItem, AccordionButton};

pub mod plotly_example;
pub mod plotly_pt1;
use crate::plotly_example::PlotlyExample;
use crate::plotly_pt1::PlotlyPT1;


#[function_component]
fn App() -> Html {
        html! {
            <Accordion
            expanded_element={html! {<AccordionButton class={"bg-blue-500 text-white p-2 rounded"}>{ "Hide -" }</AccordionButton>}}
            collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded"}>{ "Show +" }</AccordionButton>}}
            size="sm"
            aria_controls="example-accordion"
            container_class="my-custom-class bg-gray-800 p-4 rounded border border-gray-700"
            expanded_element_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
            content_container_class="my-content-class bg-gray-900 p-4 rounded border-t border-gray-700"
        >
            <ul>
                <AccordionItem
                    item_class="my-list-item-class border-b p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
                > <PlotlyExample /> </AccordionItem>
                <AccordionItem
                    item_class="my-list-item-class border-b p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
                > <PlotlyPT1 /></AccordionItem>
                <AccordionItem
                    item_class="my-list-item-class p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
                >{ "Item 3" }</AccordionItem>
            </ul>
        </Accordion>
}
}

fn main() {
    yew::Renderer::<App>::new().render();
}