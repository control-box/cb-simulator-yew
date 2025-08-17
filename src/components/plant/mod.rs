pub mod element;
pub mod element_select;
pub mod named_element_dialog;
pub mod pt1;

use cb_simulation_util::plant::BoxedTransferTimeDomain;
use yew::prelude::*;

#[derive(Properties)]
pub struct BoxedElementDialogProps {
    pub element: BoxedTransferTimeDomain<f64>,
    pub on_update: Callback<BoxedTransferTimeDomain<f64>>,
}

// explicit implementation because PartialEq via derive requires the Copy bound
// Copy bound cannot be implemented for Boxed objects
impl PartialEq for BoxedElementDialogProps {
    fn eq(&self, other: &Self) -> bool {
        self.element.clone() == other.element.clone() && self.on_update == other.on_update
    }
}
