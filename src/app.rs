use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::theme_toggle::ThemeToggle;
use crate::router::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <BrowserRouter>
          <Navbar />
          <main class="p-4">
            <Switch<Route> render={switch} />
          </main>
      </BrowserRouter>
    }
}

// ----- Menüleiste -----
#[function_component(Navbar)]
fn navbar() -> Html {
    html! {
        <nav class="bg-white dark:bg-gray-800 shadow text-gray-800 dark:text-gray-200 p-4">
          <div class="container mx-auto flex justify-between items-center">
            <ul class="flex space-x-4">
                <li><Link<Route> to={Route::TimeDomain} classes="hover:underline" >{ "Time Domain" }</Link<Route>></li>
                <li><Link<Route> to={Route::ZDomain} classes="hover:underline">{ "Z-Transformed Domain" }</Link<Route>></li>
                <li><Link<Route> to={Route::About} classes="hover:underline">{ "About" }</Link<Route>></li>
            </ul>
            <ThemeToggle />
          </div>
        </nav>
    }
}
