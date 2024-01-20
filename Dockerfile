FROM rust as build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli

WORKDIR /build

COPY src src
COPY Cargo.* ./

RUN cargo build --release --target wasm32-unknown-unknown
RUN wasm-bindgen --out-name boids \
    --out-dir target/wasm \
    --target web target/wasm32-unknown-unknown/release/bevy-boids.wasm

FROM httpd:2.4

COPY ./index.html /usr/local/apache2/htdocs/
COPY --from=build /build/target/wasm /usr/local/apache2/htdocs/target/wasm/
