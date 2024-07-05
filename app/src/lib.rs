use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <Body class="h-screen bg-ctp-base text-ctp-text"/>

        <script src="/preline/preline.js"></script>

        // <Body class="bg-ctp-base"/>
        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Routes>
                <Route path="/" view=HomePage/>
            </Routes>
        </Router>
    }
}
