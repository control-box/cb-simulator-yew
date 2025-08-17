pub mod app;
pub mod components;
mod pages;
mod plant;
mod router;
mod time_signal;

use plant::register_all::register_build_in_elements;
use time_signal::register_build_in_time_signals;

pub fn register_build_in() {
    register_build_in_elements();
    register_build_in_time_signals();
}
