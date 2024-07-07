use super::NavBar;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <NavBar bar_title="Home".into()/>
        <div class="max-w-[50-rem] flex flex-col mx-auto w-full h-full items-center justify-center">
            <main id="content">
                <h1 class="block">Ritzier Wheat</h1>
                <div class="mt-18 flex justify-cetner items-center gap-2 sm:flox-row sm:gap-3 space-x-2">
                    <a
                        class="hover:-translate-y-1 inline-flex justify-center items-center"
                        href="https://github.com/ritzier"
                    >
                        <img src="/svg/github.svg" alt="Github page" width="16" height="16"/>
                    </a>

                    <a
                        class="hover:-translate-y-1 inline-flex justify-center items-center"
                        href="https://github.com/ritzier"
                    >
                        <img src="/svg/steam.png" alt="Github page" width="16" height="16"/>
                    </a>

                    <a
                        class="hover:-translate-y-1 inline-flex justify-center items-center"
                        href="https://github.com/ritzier"
                    >
                        <img src="/svg/discord.svg" alt="Github page" width="20" height="20"/>
                    </a>

                </div>

            </main>
        </div>
    }
}
