use leptos::*;
use leptos_router::{use_location, use_route, use_router};
use logging::log;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    let location = use_location();
    log!("404 location: {:?}", location.pathname.get());
    let route = use_route();
    log!("404 route: {}, original_route: {}", route.path(), route.original_path());
    let router = use_router();
    log!("404 route: {}, 
        base path: {},
        base original path: {}
        ", router.pathname().get(), router.base().path(), router.base().original_path());
    view! { <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>  }
}
