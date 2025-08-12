use yew::prelude::*;

use crate::components::plot_time_signal::AccordeonPlotTimeSignal;
use crate::components::pt1_time_domain::PlotlyPT1;
use crate::components::time_range::AccordeonTimeRange;
use crate::components::time_signal::time_signal::AccordeonTimeSignals;

use control_box::signal::{NamedTimeSignal, TimeRange};

#[function_component(TimeDomain)]
pub fn time_domain() -> Html {
    let time_range_handle = use_state(|| TimeRange::default());
    let time_range = (*time_range_handle).clone();

    let signals_handle = use_state(|| Vec::<NamedTimeSignal<f64>>::new());
    let signals = (*signals_handle).clone();

    html! {
        <>
            <AccordeonTimeRange handle={time_range_handle}/>
            <AccordeonTimeSignals signals={signals_handle} />
            <AccordeonPlotTimeSignal range={time_range.clone()} signals={signals} />
            <PlotlyPT1 />
        </>
    }
}
