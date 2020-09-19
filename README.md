# Bevy + miniquad experiment

#### pre-req

    $ rustup target add wasm32-unknown-unknown
    info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
    info: installing component 'rust-std' for 'wasm32-unknown-unknown'
    $ cargo install wasm-bindgen-cli

#### build & run

    $ cargo build --release --target wasm32-unknown-unknown
    $ wasm-bindgen --out-dir target --target web target/wasm32-unknown-unknown/release/bevy_miniquad.wasm
    $ sed -i 's/import.*from .env.;/init.set_wasm = w => wasm = w;/;s/imports\[.env.\] =.*/return imports;/' target/bevy_miniquad.js

Then serve project dir to browser. i.e.

    $ basic-http-server .
    [INFO ] basic-http-server 0.8.1
    [INFO ] addr: http://127.0.0.1:4000
    [INFO ] root dir: .
