use crate::components::counter_btn::Button;
use leptos::*;
use leptos_router::{use_location, use_route, use_router};
use logging::log;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let location = use_location();
    log!("home location: {:?}", location.pathname.get());
    let route = use_route();
    log!("home route: {}, original_route: {}", route.path(), route.original_path());
    let router = use_router();
    log!("home route: {}, 
        base path: {},
        base original path: {}
        ", router.pathname().get(), router.base().path(), router.base().original_path());

    log!("home route: {}, original_route: {}", route.path(), route.original_path());
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>"Welcome to my site!"</h1>
                <p>"A site written in Rust using "<a href="https://github.com/leptos-rs/leptos" target="_blank">"Leptos"</a>" deployed on Github Pages"</p>
                <p>"The backend will also be written in Rust and deployed on AWS Lambda"</p>
                <div class="buttons">
                    <Button/>
                    <Button increment=5/>
                </div>

            </div>
        </ErrorBoundary>
    }
}
