use yew::prelude::*;


#[cfg(not(feature = "customized_about"))]
const GIT_TAG: &str = env!("GIT_TAG");

#[cfg(feature = "customized_about")]
#[function_component(CustomizedAbout)]
pub fn customized_about() -> Html {
    html! {
        <div>
            <p>{ "This version is customized or extended" }</p>
        </div>
    }
}

#[cfg(not(feature = "customized_about"))]
#[function_component(CustomizedAbout)]
pub fn development_about() -> Html {
    html! {
        <div>
            <p>{ format!("This is a development version: {}", GIT_TAG) }</p>
            <p>{ "It is self-containt and not customized or extended" }</p>
        </div>
    }
}
