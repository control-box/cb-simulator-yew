use yew::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use gloo::utils::document;
use log::info;


#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {

    let dark_mode = use_state(|| {
        LocalStorage::get("theme").unwrap_or_else(|_| "light".to_string()) == "dark"
    });

    {
        let dark_mode = dark_mode.clone();
        use_effect_with((), move |_| {
            if let Some(doc_el) = document().document_element() {
                let class_list = doc_el.class_list();
                if *dark_mode {
                    class_list.add_1("dark").unwrap();
                } else {
                    class_list.remove_1("dark").unwrap();
                }
            }
            || {}
        });
    }

    let toggle = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            let new_value = !*dark_mode;
            dark_mode.set(new_value);
            LocalStorage::set("theme", if new_value { "dark" } else { "light" }).unwrap();
            info!("Theme is dark mode now: {}", new_value);

            if let Some(doc_el) = document().document_element() {
                let class_list = doc_el.class_list();
                if new_value {
                    info!("Setting dark mode");
                    class_list.add_1("dark").unwrap();
                } else {
                    info!("Setting light mode");
                    class_list.remove_1("dark").unwrap();
                }
            }
        })
    };

    html! {
        <button onclick={toggle}
            class="px-3 py-1 rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 transition">
            { if *dark_mode { "🌙 Dark" } else { "☀️ Light" } }
        </button>
    }
}
