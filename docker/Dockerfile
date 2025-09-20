FROM --platform=linux/amd64 rust:1.90-bullseye

RUN apt update && \
    apt install -y \
    qemu-system-x86-64 && \
    rm -rf /var/lib/apt/lists/*

RUN rustup default nightly
RUN rustup target add x86_64-unknown-none
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN rustup component add llvm-tools-preview
RUN rustup component add clippy
RUN rustup component add rustfmt
RUN cargo install bootimage
