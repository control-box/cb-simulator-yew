use crate::components::time_signal::impulse_fn;
use crate::components::time_signal::step_fn;

pub fn register_build_in_time_signals() {
    step_fn::register();
    impulse_fn::register();
}
