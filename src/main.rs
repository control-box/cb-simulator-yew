use cb_simulator_yew::*;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    register_build_in();

    yew::Renderer::<app::App>::new().render();
}
