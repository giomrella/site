use leptos::*;
use leptos_router::{use_location, use_navigate};

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();
    if location.pathname.get() == "/site/" {
        navigate("", Default::default());
    }

    view! { <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>  }
}
