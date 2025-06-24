use yew::prelude::*;

use crate::components::plotly_example::PlotlyExample;
use crate::components::pt1_time_domain::PlotlyPT1;
use crate::components::time_range::AccordeonTimeRange;
use crate::components::plot_time_signal::AccordeonPlotTimeSignal;

use control_box::signal::{TimeRange, StepFunction};

#[function_component(TimeDomain)]
pub fn time_domain() -> Html {

    let time_range_handle = use_state(TimeRange::default);
    let time_range = (*time_range_handle).clone();

    let signal = StepFunction::default().step(10.0).post(1.0);
    // let signal = Box::new(signal);

    html! {
        <>
            <AccordeonTimeRange handle={time_range_handle}/>
            <AccordeonPlotTimeSignal range={time_range} signal={signal}/>
            <PlotlyExample />
            <PlotlyPT1 />
        </>
    }
}
