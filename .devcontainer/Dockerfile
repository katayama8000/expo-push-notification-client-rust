FROM rust:1.93.0-bookworm

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN rustup component add rustfmt clippy
RUN cargo install cargo-update
RUN cargo install cargo-llvm-cov
RUN rustup component add llvm-tools-preview
