use yew::prelude::*;

const GIT_HASH: &str = env!("GIT_HASH");
const GIT_TAG: &str = env!("GIT_TAG");
const CRATE_VERSION: &str = env!("CRATE_VERSION");

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>

            <h2 class="text-2xl">{ "About Control-Box Simulator" }</h2>

            <div class="">
                <p>{ "This simulator allows you to explore control systems in the time and z-domain. " }</p>
                <p>{ "You can simulate and visualize the behavior of various control systems. " }</p>
                <p>{ "Feel free to contribute to the project on GitHub! " }</p>
            </div>
            <hr class="my-4 h-0.5 border-t-0 bg-gray-400 dark:bg-gray-600" />
            <div class="">
                <p>{ "Copyright (c) 2025 almedso GmbH" }</p>
                <p>{ "License: MIT" }</p>
                <p>{ "Contact: info@almedso.de" }</p>
            </div>
            <hr class="my-4 h-0.5 border-t-0 bg-gray-400 dark:bg-gray-600" />
            <div class="">
                <p>{ format!("Git Commit: {}", GIT_HASH) }</p>
                <p>{ format!("Git Tag: {}", GIT_TAG) }</p>
                <p>{ format!("Crate Version: {}", CRATE_VERSION) }</p>
            </div>

        </div>
    }
}
