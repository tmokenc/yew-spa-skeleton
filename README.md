# Yew SPA project skeleton
A skeleton project to start a new Yew web single page app (SPA)<br>

## Why
Everytime I create new project with Yew web framework, it always ended up at this state no matter what

## Requirement
To build wasm file, we will need `rust`, `wasm-pack`, `wasm-bindgen-cli`, `wasm-opt`

Install Rust
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install wasm-pack
```sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Install wasm-bindgen-cli
```sh
cargo install -f wasm-bindgen-cli
```

Install wasm-opt, it's lies under `binaryen` package
```sh
sudo pacman -S binaryen
```

## Build
Just simple run the `build_wasm.sh` file
```sh
sh build_wasm.sh
```

## Run
After the wasm are built, run the server with
```
cargo run --release
```

We can change the `IP` and `port` in the file `config.toml`

## Server
The server can be anything, I just use `actix-web` because I got familiar with it<br>
It needs to have 3 basic `GET` routes to serve the web
- `/static` - serve files in `web/static/`
- `/wasm` - serve files in `web/pkg`, which is wasm file when they built
- `*` - any other route will point to the `web/index.html` file, the `yew-router` in the frontend will handle the rest