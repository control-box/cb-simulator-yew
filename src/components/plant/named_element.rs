use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;

use std::{borrow::ToOwned, boxed::Box, string::String};

use control_box::plant::TypeIdentifier;
use control_box::plant::{pt1::PT1, BoxedTransferTimeDomain, TransferTimeDomain};

#[derive(Debug, Clone)]
pub struct NamedElement<S: Debug + Display + Clone + Copy + PartialEq + 'static> {
    pub name: String,
    pub element: BoxedTransferTimeDomain<S>,
}

impl<S: Debug + Display + Clone + Copy + PartialEq + 'static> NamedElement<S> {
    pub fn set_name(self, name: String) -> Self {
        NamedElement { name, ..self }
    }

    pub fn set_element(self, element: BoxedTransferTimeDomain<S>) -> Self {
        NamedElement { element, ..self }
    }
}

impl<S: Debug + Display + Clone + Copy + PartialEq + 'static + Send + Sync> PartialEq
    for NamedElement<S>
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.element == other.element.clone()
    }
}

impl<S: Debug + Display + Clone + Copy + PartialEq> fmt::Display for NamedElement<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Time Signal: {} = {}", self.name, self.element)
    }
}

impl<S: Debug + Display + Clone + Copy + PartialEq + 'static + Send + Sync> Default
    for NamedElement<S>
where
    PT1<f64>: TransferTimeDomain<S>,
{
    fn default() -> Self {
        let default = PT1::<f64>::default();
        NamedElement {
            name: default.short_type_name().to_owned(),
            element: Box::new(default) as BoxedTransferTimeDomain<S>,
        }
    }
}
