use leptos::{html::*, *};
use leptos_use::{
    on_click_outside, use_color_mode_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn,
};

use super::svg_icons::ThickIcon;

#[component]
pub fn NavBar() -> impl IntoView {
    // PAGE of Popup
    // Colorscheme: latte, frappe, macchiato, mocha
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .cookie_enabled(true)
            .custom_modes(vec![
                "ctp-latte".into(),
                "ctp-frappe".into(),
                "ctp-macchiato".into(),
                "ctp-mocha".into(),
            ]),
    );

    provide_context((mode, set_mode));

    // LOGO Colorscheme Path
    view! {
        <nav class="flex justify-between px-8 gap-4 ">
            <div class="basis-1/4">
                <a href="/" class="gap-12 text-white font-bold">
                    Logo
                </a>
            </div>
            <div class="flex items-center justify-end">
                <div class="space-x-3">
                    <a href="#">"Blog"</a>
                    <a href="#">"Demo"</a>
                    <a href="#">"About"</a>
                </div>

                <div class="ps-2 text-sm">
                    <ColorschemeButton/>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn ColorschemeButton() -> impl IntoView {
    // Button to show the popup page
    let (show_popup, set_show_popup) = create_signal(false);
    let popup_ref = create_node_ref::<Div>();
    let _ = on_click_outside(popup_ref, move |_| set_show_popup.set(false));

    view! {
        <button on:click=move |_| set_show_popup.set(true)>
            <svg
                // style="height:1em;font-size:1.5em;vertical-align:-.125em;transform-origin:center;overflow:visible"
                style="height:0.8em;"
                viewBox="0 0 512 512"
                role="img"
                xmlns="http://www.w3.org/2000/svg"
            >
                <g transform="translate(256 256)" transform-origin="128 0">
                    <g transform="translate(0,0) scale(1,1)">
                        <path
                            d="M512 256c0 .9 0 1.8 0 2.7c-.4 36.5-33.6 61.3-70.1 61.3H344c-26.5 0-48 21.5-48 48c0 3.4 .4 6.7 1 9.9c2.1 10.2 6.5 20 10.8 29.9c6.1 13.8 12.1 27.5 12.1 42c0 31.8-21.6 60.7-53.4 62c-3.5 .1-7 .2-10.6 .2C114.6 512 0 397.4 0 256S114.6 0 256 0S512 114.6 512 256zM128 288a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm0-96a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM288 96a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm96 96a32 32 0 1 0 0-64 32 32 0 1 0 0 64z"
                            fill="currentColor"
                            transform="translate(-256 -256)"
                        ></path>
                    </g>
                </g>
            </svg>
        </button>

        <Show when=move || show_popup.get() fallback=|| ()>
            // Modol
            <ColorschemePopupPageOne popup_ref=popup_ref set_show=set_show_popup/>
        </Show>
    }
}

#[component]
fn ColorschemePopupPageOne(popup_ref: NodeRef<Div>, set_show: WriteSignal<bool>) -> impl IntoView {
    let (mode, set_mode) =
        use_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>().expect("Not found ColorMode");

    view! {
        <div class="fixed inset-0 bg-gray-500 bg-opacity-25 flex items-center justify-center">
            <div class="bg-ctp-base flex items-center justify-center" node_ref=popup_ref>
                <div class="p-4 shadow-lg rounded-lg text-center rounded-lg">
                    <h2 class="text-2xl font-bold mb-4">"Colorscheme For Experience"</h2>

                    <div class="grid grid-cols-1 gap-4 justify-center items-center">
                        <button
                            on:click=move |_| {
                                set_mode.set(ColorMode::Custom("ctp-latte".into()))
                            }

                            class="w-full sm:max-w-80 py-2 px-4 text-center bg-gradient-to-r from-ctp-pink to-ctp-peach hover:bg-ctp-sky rounded focus:ring-2 focus:ring-offset-2 focus:ring-ctp-sky"
                        >
                            <span class="flex items-center justify-center text-ctp-crust">
                                Latte
                                <Show
                                    when=move || mode.get() == ColorMode::Custom("ctp-latte".into())
                                    fallback=|| ()
                                >
                                    <ThickIcon/>
                                </Show>
                            </span>
                        </button>

                        <button
                            on:click=move |_| {
                                set_mode.set(ColorMode::Custom("ctp-frappe".into()))
                            }

                            class="w-full sm:max-w-80 py-2 px-4 text-center bg-gradient-to-r from-ctp-sapphire to-ctp-lavender hover:bg-gray-100 rounded focus:ring-2 focus:ring-offset-2 focus:ring-primary"
                        >
                            <span class="flex items-center justify-center text-ctp-crust">
                                Frappe
                                <Show
                                    when=move || {
                                        mode.get() == ColorMode::Custom("ctp-frappe".into())
                                    }

                                    fallback=|| ()
                                >
                                    <ThickIcon/>
                                </Show>
                            </span>
                        </button>

                        <button
                            on:click=move |_| {
                                set_mode.set(ColorMode::Custom("ctp-macchiato".into()))
                            }

                            class="w-full sm:max-w-80 py-2 px-4 text-center bg-gradient-to-r from-ctp-mauve to-ctp-maroon hover:bg-gray-100 rounded focus:ring-2 focus:ring-offset-2 focus:ring-primary"
                        >
                            <span class="flex items-center justify-center text-ctp-crust">
                                Macchiato
                                <Show
                                    when=move || {
                                        mode.get() == ColorMode::Custom("ctp-macchiato".into())
                                    }

                                    fallback=|| ()
                                >
                                    <ThickIcon/>
                                </Show>
                            </span>
                        </button>

                        <button
                            on:click=move |_| {
                                set_mode.set(ColorMode::Custom("ctp-mocha".into()))
                            }

                            class="w-full sm:max-w-80 py-2 px-4 text-center bg-gradient-to-r from-ctp-mantle to-ctp-red hover:bg-gray-100 rounded focus:ring-2 focus:ring-offset-2 focus:ring-primary"
                        >
                            <span class="flex items-center justify-center text-ctp-crust">
                                Mocha
                                <Show
                                    when=move || mode.get() == ColorMode::Custom("ctp-mocha".into())
                                    fallback=|| ()
                                >
                                    <ThickIcon/>
                                </Show>
                            </span>
                        </button>

                    </div>

                    <button
                        on:click=move |_| set_show.set(false)
                        class="bg-blue-500 hover:bg-blue-600 text-white font-bold mt-4 py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    >
                        Done
                    </button>
                </div>
            </div>

        </div>
    }
}