
use yew::prelude::*;
use log::info;

use crate::components::control::controller::AccordeonController;
use crate::components::plant::element::AccordeonElements;
use crate::components::plot_element::AccordeonPlotElement;
use crate::components::plot_control::AccordeonPlotControl;
use crate::components::plot_time_signal::AccordeonPlotTimeSignal;
use crate::components::time_range::AccordeonTimeRange;
use crate::components::time_signal::time_signal::AccordeonTimeSignals;

use crate::plant::named_element::NamedElement;
use crate::time_signal::named_time_signal::NamedTimeSignal;
use cb_simulation_util::signal::TimeRange;

use cb_controller::pid::{PidController, PidCoreBuilder};

#[function_component(TimeDomain)]
pub fn time_domain() -> Html {
    let time_range_handle = use_state(|| TimeRange::default());
    let time_range = (*time_range_handle).clone();

    let signals_handle = use_state(|| Vec::<NamedTimeSignal<f64>>::new());
    let signals = (*signals_handle).clone();

    let elements_handle = use_state(|| Vec::<NamedElement<f64>>::new());
    let elements = (*elements_handle).clone();

    let controller_handle = use_state(|| PidCoreBuilder::<f64>::default()
        .sampling_interval(time_range.sampling_interval.clone() as f32)
        .build());

    let on_controller_update  = {
        let controller_handle = controller_handle.clone();
        Callback::from(move |updated: PidController<f64>| {
            info!("Time domain - new Controller: {:?}", updated);
            controller_handle.set(updated)
        })
    };
    html! {
        <>
            <AccordeonTimeRange handle={time_range_handle}/>

            <AccordeonTimeSignals signals={signals_handle} />
            <AccordeonPlotTimeSignal range={time_range.clone()} signals={signals.clone()} />

            <AccordeonElements elements={elements_handle} sample_time={time_range.sampling_interval.clone()} />
            <AccordeonPlotElement range={time_range.clone()} signals={signals.clone()} elements={elements.clone()} />

            <AccordeonController update={on_controller_update.clone()} sampling_interval={time_range.sampling_interval.clone()} />
            <AccordeonPlotControl range={time_range.clone()} signals={signals.clone()} elements={elements.clone()} controller={(*controller_handle).clone()} />

        </>

    }
}
