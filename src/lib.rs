use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
#[allow(unused)]
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="gio"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router base="site">
			<nav>
			  <A href="">"Home"</A>"-"
			  <A href="about">"About"</A>"-"
			  <A href="contact">"Contact"</A>
			</nav>
            <Routes base="site".to_owned()>
                <Route path="" view=Home/>
                <Route path="/" view=Home/>
                <Route path="//" view=Home/>
                <Route path="site" view=Home/>
                <Route path="site/" view=Home/>
                <Route path="/site" view=Home/>
                <Route path="/site/" view=Home/>
                <Route path="/about" view=About/>
                <Route path="/contact" view=Contact/>
                <Route path="/*" view=NotFound/>
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
