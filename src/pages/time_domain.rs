use yew::prelude::*;

use std::borrow::BorrowMut;


use crate::components::plot_time_signal::AccordeonPlotTimeSignal;
use crate::components::pt1_time_domain::PlotlyPT1;
use crate::components::time_range::AccordeonTimeRange;
use crate::components::time_signal::AccordeonTimeSignals;

use control_box::signal::{NamedTimeSignal, StepFunction, TimeRange};

#[function_component(TimeDomain)]
pub fn time_domain() -> Html {
    let time_range_handle = use_state(|| TimeRange::default);
    let time_range = (*time_range_handle).clone();

    let signals_handle = use_state( || vec![
        NamedTimeSignal::<f64>::default(),
        NamedTimeSignal::<f64>::default(),
    ]);


    let on_add = Callback::from(  move |new: NamedTimeSignal<f64>| {
        // let mut signals = signals_handle.borrow_mut().push(new);
    });

    // let on_remove = Callback::from(move |signal_index| {
    //     let index: usize = signal_index.try_into();
    //     signals.remove(index);
    // });

    // let signal = signals[0].signal.clone();
    let signal = StepFunction::<f64>::default();

    html! {
        <>
            <AccordeonTimeRange handle={time_range_handle}/>
            // <AccordeonTimeSignals signals={signals_handle} on_add={on_add} />
            <AccordeonPlotTimeSignal range={time_range()} signal={signal}/>
            <PlotlyPT1 />
        </>
    }

}
