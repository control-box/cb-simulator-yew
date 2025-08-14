use control_box::plant::DynTransferTimeDomain;
use yew::{Callback, Html};

use once_cell::sync::Lazy;
use std::sync::RwLock;

pub trait YewElement: Send + Sync {
    fn dialog(
        &self,
        element: Box<dyn DynTransferTimeDomain<f64>>,
        on_update: Callback<Box<dyn DynTransferTimeDomain<f64>>>,
    ) -> Html;
    fn name(&self) -> &str;
    fn render(&self) -> Html;
    fn element(&self) -> Box<dyn DynTransferTimeDomain<f64> + 'static + Sync + Send>;
}

type ElementFactory = fn() -> Box<dyn YewElement + Sync>;

pub static ELEMENT_REGISTRY: Lazy<RwLock<Vec<ElementFactory>>> =
    Lazy::new(|| RwLock::new(Vec::new()));

pub fn register_element(factory: ElementFactory) {
    let mut map = ELEMENT_REGISTRY.write().expect("registry write lock");
    map.push(factory);
}

pub fn list_factories() -> Vec<ElementFactory> {
    let map = ELEMENT_REGISTRY.read().expect("registry read lock");
    map.to_vec()
}
