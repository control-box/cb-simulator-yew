use yew::prelude::*;

use crate::components::plot_time_signal::AccordeonPlotTimeSignal;
use crate::components::pt1_time_domain::PlotlyPT1;
use crate::components::time_range::AccordeonTimeRange;
use crate::components::time_signal::time_signal::AccordeonTimeSignals;
use crate::components::plot_element::AccordeonPlotElement;
use crate::components::plant::element::AccordeonElements;

use control_box::signal::{NamedTimeSignal, TimeRange};
use crate::components::plant::named_element::NamedElement;

#[function_component(TimeDomain)]
pub fn time_domain() -> Html {
    let time_range_handle = use_state(|| TimeRange::default());
    let time_range = (*time_range_handle).clone();

    let signals_handle = use_state(|| Vec::<NamedTimeSignal<f64>>::new());
    let signals = (*signals_handle).clone();

    let elements_handle = use_state(|| Vec::<NamedElement<f64>>::new());
    let elements = (*elements_handle).clone();

    html! {
        <>
            <AccordeonTimeRange handle={time_range_handle}/>
            <AccordeonTimeSignals signals={signals_handle} />
            <AccordeonElements elements={elements_handle} />

            <AccordeonPlotTimeSignal range={time_range.clone()} signals={signals.clone()} />
            <AccordeonPlotElement range={time_range.clone()} signals={signals} elements={elements} />
            <PlotlyPT1 />

        </>

    }

}
