use crate::components::counter_btn::Button;
use leptos::*;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
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
