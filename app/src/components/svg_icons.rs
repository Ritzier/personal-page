use leptos::*;

#[component]
pub fn ThickIcon() -> impl IntoView {
    //! TODO:
    //! <ThickIcon/>
    //! <ThickIcon class="text-white">
    //! as Leptos HTMLElement

    view! {
        <svg
            class="ml-2 h-4 w-4 text-base-900"
            style="height:1em;font-size:.75em;vertical-align:-.125em;transform-origin:center;overflow:visible"
            viewBox="0 0 448 512"
            role="img"
            xmlns="http://www.w3.org/2000/svg"
        >
            <g transform="translate(224 256)" transform-origin="112 0">
                <g transform="translate(0,0) scale(1,1)">
                    <path
                        d="M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z"
                        fill="currentColor"
                        transform="translate(-224 -256)"
                    ></path>
                </g>
            </g>
        </svg>
    }
}
