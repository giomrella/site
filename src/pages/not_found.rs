use leptos::*;
use leptos_router::{use_location, use_route};

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    let location = use_location();
    println!("location: {location:?}");
    let route = use_route();
    println!("route: {route:?}");
    view! { <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>  }
}
