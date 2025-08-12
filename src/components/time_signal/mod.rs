pub mod impulse_fn;
pub mod named_time_signal_dialog;
pub mod register_all;
pub mod registry;
pub mod step_fn;
pub mod time_signal;
pub mod time_signal_select;

pub use register_all::register_build_in_time_signals;

use control_box::signal::BoxedTimeSignal;
use yew::prelude::*;

#[derive(Properties)]
pub struct BoxedTimeSignalDialogProps {
    pub time_signal: BoxedTimeSignal<f64>,
    pub on_update: Callback<BoxedTimeSignal<f64>>,
}

// explicit implementation because PartialEq via derive requires the Copy bound
// Copy bound cannot be implemented for Boxed objects
impl PartialEq for BoxedTimeSignalDialogProps {
    fn eq(&self, other: &Self) -> bool {
        self.time_signal.clone() == other.time_signal.clone() && self.on_update == other.on_update
    }
}
