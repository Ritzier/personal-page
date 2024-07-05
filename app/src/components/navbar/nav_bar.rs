use leptos::{html::*, *};
use leptos_use::{
    on_click_outside, use_color_mode_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn,
};

use super::svg_icons::{ThemeIcon, ThickIcon};

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
    //! Icon, Button call <ColorschemePopupPage/>
    let (show_popup, set_show_popup) = create_signal(false);
    let popup_ref = create_node_ref::<Div>();
    let _ = on_click_outside(popup_ref, move |_| set_show_popup.set(false));

    view! {
        <button on:click=move |_| set_show_popup.set(true)>
            <ThemeIcon/>
        </button>

        <Show when=move || show_popup.get() fallback=|| ()>
            // Modol
            <ColorschemePopupPage popup_ref=popup_ref set_show=set_show_popup/>
        </Show>
    }
}

#[component]
fn ColorschemePopupPage(popup_ref: NodeRef<Div>, set_show: WriteSignal<bool>) -> impl IntoView {
    //! Popup page of theme selection and make background-gray opacity-25
    let (mode, set_mode) =
        use_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>().expect("Not found ColorMode");

    view! {
        <div class="fixed inset-0 bg-gray-500 bg-opacity-50 flex items-center justify-center">
            <div class="bg-ctp-base flex items-center justify-center" node_ref=popup_ref>
                <div class="p-4 shadow-lg rounded-lg text-center rounded-lg">
                    <h2 class="text-2xl font-bold mb-4 bg-gradient-to-r from-ctp-pink to-ctp-mauve text-transparent bg-clip-text">
                        Colorscheme For Experience
                    </h2>

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
