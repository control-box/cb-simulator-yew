mod app;
mod components;
mod pages;
mod router;

use components::plant::register_all::register_build_in_elements;
use components::time_signal::register_build_in_time_signals;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    register_build_in_time_signals();
    register_build_in_elements();

    yew::Renderer::<app::App>::new().render();
}
