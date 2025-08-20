use yew::prelude::*;


const GIT_TAG: &str = env!("GIT_TAG");

#[function_component(AboutGit)]
pub fn about_git() -> Html {
    html! {
        <div>
            { format!("Git describe of cb-simulator-yew: {}", GIT_TAG) }
        </div>
    }
}
