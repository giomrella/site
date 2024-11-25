use leptos::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::{Router,Routes,Route}, path};

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

// components
use crate::components::nav::Nav;

#[allow(unused)]

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="gio"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        // <Router>
        <Router base="/site">
            <Nav />
            <Routes fallback=NotFound>
            // <ParentRoute path=path!("site") view=Home>
                <Route path=path!("") view=Home/>
                <Route path=path!("/about") view=About/>
                <Route path=path!("/contact") view=Contact/>
                // <Route path=path!("/*") view=NotFound/>
            // </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
pub fn About() -> impl IntoView {
    view! { <h1>"About"</h1>  }
}

#[component]
pub fn Contact() -> impl IntoView {
    view! { <h1>"Contact"</h1>  }
}
