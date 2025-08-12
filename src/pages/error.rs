use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! { <h2>{ "Page not found 404 - error" }</h2> }
}
