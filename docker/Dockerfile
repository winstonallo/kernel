FROM --platform=linux/amd64 rust:1.90-bullseye

RUN apt update && \
    apt install -y \
    qemu-system-x86-64 && \
    rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-none

RUN rustup component add rust-src
