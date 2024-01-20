

build-wasm:
    @echo "Building wasm"
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --out-name boids \
        --out-dir target/wasm \
        --target web target/wasm32-unknown-unknown/release/bevy-boids.wasm

serve: build-wasm
    npx serve
