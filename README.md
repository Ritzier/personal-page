<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum Workspace

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool using [Axum](https://github.com/tokio-rs/axum).

## Run it

Install npm packages and compile it:

```bash
npm install
npm run build
```

Rust dependencies

```bash
cargo install cargo-leptos
rustup target add wasm32-unknown-unknown
```

Start leptos server

```bash
cargo leptos watch
```

## Development:

Start the server:

```bash
cargo leptos watch
```

Live Compile Tailwindcss:

```bash
npm run watch
```

## TODO

- Blog page and render `md` file

- Demo page:

  - (Topaz)[https://www.topazlabs.com/]

- About page:

  - Github link
  - mokeytype link

- Style Guide

- Docker
