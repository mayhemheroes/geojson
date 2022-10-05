FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD ../ /geojson
WORKDIR /geojson/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /geojson/fuzz/target/x86_64-unknown-linux-gnu/release/geojson-fuzz /