use crate::components::plant::pt1;
use crate::components::plant::pt2;

pub fn register_build_in_elements() {
    pt1::register();
    pt2::register();
}
