use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::error::Error;
use crate::pages::landing::Landing;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/error")]
    Error,
    #[at("/")]
    Landing,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Landing => html! { <Landing /> },
        Route::Error => html! { <Error /> },
    }
}
