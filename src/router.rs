use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::about::About;
use crate::pages::error::NotFound;
use crate::pages::time_domain::TimeDomain;
use crate::pages::z_domain::ZDomain;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    TimeDomain,
    #[at("/z-domain")]
    ZDomain,
    #[at("/about")]
    About,
    #[at("/404")]
    #[not_found]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::TimeDomain => html! { <TimeDomain /> },
        Route::ZDomain => html! { <ZDomain /> },
        Route::About => html! { <About /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
