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

#[component]
pub fn ThemeIcon() -> impl IntoView {
    view! {
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
    }
}
