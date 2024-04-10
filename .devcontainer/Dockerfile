FROM rust:1.77.2-bookworm

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN rustup component add rustfmt clippy
RUN cargo install cargo-update
