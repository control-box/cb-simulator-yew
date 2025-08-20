use crate::components::plant::pt0;
use crate::components::plant::pt1;
use crate::components::plant::pt2;

pub fn register_build_in_elements() {
    pt0::register();
    pt1::register();
    pt2::register();
}
