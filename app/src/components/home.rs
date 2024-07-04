use super::NavBar;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <NavBar/>
        <h3 class="bg-gradient-to-r from-ctp-pink to-ctp-mauve inline-block text-transparent bg-clip-text">
            Fuck
        </h3>
    }
}
