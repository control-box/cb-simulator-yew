
use ndarray::{Array, Ix1};
use plotly::{Layout, Scatter, layout::Axis};
use yew::prelude::*;
use yew_accordion::{Accordion, AccordionButton, AccordionItem};
use yew_plotly::Plotly;
use yew_plotly::plotly::Plot;
use yew_plotly::plotly::common::Mode;

use control_box::signal::{StepFunction, TimeRange, TimeSignal};

#[derive(Properties, PartialEq)]
pub struct TimeSignalProps {
    #[prop_or_default]
    pub range: TimeRange,
    #[prop_or_default]
    pub signal: StepFunction::<f64>,
}

#[function_component(PlotTimeSignal)]
pub fn plotly_time_signal(props: &TimeSignalProps) -> Html {
    let time: Array<f64, Ix1> = props.range.collect();
    //let s = StepFunction::default();
    let signal: Array<f64, Ix1> = time.iter().map(|v| props.signal.time_to_signal(*v)).collect();

    let mut plot = Plot::new();
    let trace = Scatter::from_array(time.clone(), signal)
        .mode(Mode::LinesMarkers)
        .show_legend(false)
        .name("Time signal");
    plot.add_trace(trace);

    let layout = Layout::new()
        .title("<b>Signal in Time Domain</b>".into())
        .x_axis(
            Axis::new().title("time [ms]".into()), // plotly 0.8.3 does not support From<String>
        )
        .y_axis(Axis::new().title("Signal Aplitude".into()));
    plot.set_layout(layout);

    html! {
        <Plotly plot={plot}/>
    }
}

#[function_component(AccordeonPlotTimeSignal)]
pub fn accordeon_time_signal(props: &TimeSignalProps) -> Html {

    html! {
        <Accordion
            expanded_element={html! {<AccordionButton class={"bg-blue-500 text-white p-2 rounded"}>
                { "Plot Signal" }
            </AccordionButton>}}
            collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded"}>
                { "Plot Signal: " }
            </AccordionButton>}}

            aria_controls="example-accordion"
            container_class="my-custom-class bg-gray-800 p-4 rounded border border-gray-400"
            expanded_element_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
            content_container_class="my-content-class bg-gray-500 p-4 rounded border-t border-gray-700"
        >
            <ul>
                <AccordionItem
                    item_class="my-list-item-class border-b p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
                >
                    <PlotTimeSignal range={props.range} signal={props.signal}/>
                </AccordionItem>
            </ul>
        </Accordion>

    }
}
