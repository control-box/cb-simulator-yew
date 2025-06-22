use ndarray::{Array, Ix1};
use plotly::{Layout, Scatter, layout::Axis};
use yew::prelude::*;
use yew_plotly::Plotly;
use yew_plotly::plotly::Plot;
use yew_plotly::plotly::common::Mode;

use yew_accordion::{Accordion, AccordionButton, AccordionItem};

#[derive(Properties, PartialEq)]
pub struct PlotlyExampleProps {}

#[function_component(PlotlyExample)]
pub fn plotly_example(PlotlyExampleProps {}: &PlotlyExampleProps) -> Html {
    let n: usize = 11;
    let t: Array<f64, Ix1> = Array::range(0., 10., 10. / n as f64);
    let ys: Array<f64, Ix1> = t.iter().map(|v| (*v).powf(2.)).collect();

    let trace = Scatter::from_array(t, ys)
        .mode(Mode::LinesMarkers)
        .show_legend(true)
        .name("Scatter");

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .title("<b>Line and Scatter Plot</b>".into())
        .x_axis(Axis::new().title("time [ms]".into()));
    plot.set_layout(layout);

    html! {
        <Accordion
        expanded_element={html! {<AccordionButton class={"bg-blue-500 text-white p-2 rounded"}>{ "Hide: Plotly simple example" }</AccordionButton>}}
        collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded"}>{ "Show: Plotly simple example" }</AccordionButton>}}

        aria_controls="example-accordion"
        container_class="my-custom-class bg-gray-800 p-4 rounded border border-gray-700"
        expanded_element_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
        collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
        content_container_class="my-content-class bg-gray-900 p-4 rounded border-t border-gray-700"
    >
        <ul>
            <AccordionItem
                item_class="my-list-item-class border-b p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
            >
                <Plotly plot={plot}/>
            </AccordionItem>
        </ul>
    </Accordion>
         }
}
