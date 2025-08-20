use yew::prelude::*;
use crate::components::about_git::AboutGit;

const SIMULATOR_YEW: &str = env!("DEP_CB_SIMULATOR_YEW_VERSION");
const SIMULATION_UTIL: &str = env!("DEP_CB_SIMULATION_UTIL_VERSION");

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
                <p> <AboutGit/> </p>
                <p>{ format!("Crate cb-simulator-yew: {}", SIMULATOR_YEW) }</p>
                <p>{ format!("Crate cb-simulation-util: {}", SIMULATION_UTIL) }</p>
            </div>

        </div>
    }
}
