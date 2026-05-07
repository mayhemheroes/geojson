FROM rustlang/rust:nightly AS builder

RUN cargo install cargo-fuzz

ADD ../ /geojson
WORKDIR /geojson/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /geojson/fuzz/target/x86_64-unknown-linux-gnu/release/geojson-fuzz /