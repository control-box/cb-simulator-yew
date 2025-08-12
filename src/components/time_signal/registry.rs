use control_box::signal::DynTimeSignal;
use yew::Html;

use once_cell::sync::Lazy;
use std::sync::RwLock;

pub trait YewTimeSignal: Send + Sync {
    fn name(&self) -> &str;
    fn render(&self) -> Html;
    fn signal(&self) -> Box<dyn DynTimeSignal<f64> + 'static + Sync + Send>;
}

type TimeSignalFactory = fn() -> Box<dyn YewTimeSignal + Sync>;

pub static TIME_SIGNAL_REGISTRY: Lazy<RwLock<Vec<TimeSignalFactory>>> =
    Lazy::new(|| RwLock::new(Vec::new()));

pub fn register_time_signal(factory: TimeSignalFactory) {
    let mut map = TIME_SIGNAL_REGISTRY.write().expect("registry write lock");
    map.push(factory);
}

pub fn list_factories() -> Vec<TimeSignalFactory> {
    let map = TIME_SIGNAL_REGISTRY.read().expect("registry read lock");
    map.to_vec()
}
