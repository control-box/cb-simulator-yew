use yew::prelude::*;

use crate::components::plotly_example::PlotlyExample;
use crate::components::pt1_time_domain::PlotlyPT1;
use crate::components::time_signal::AccordeonTimeSignal;

#[function_component(Landing)]
pub fn login() -> Html {
    html! {
        <>
        <PlotlyExample />
        <PlotlyPT1 />
        <AccordeonTimeSignal/>
    </>
    }
}
