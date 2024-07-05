use super::NavBar;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! { <NavBar bar_title="Home".into()/> }
}
