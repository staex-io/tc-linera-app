FROM rust:1.86-slim-bookworm

RUN apt-get update && apt-get install -y \
    pkg-config \
    protobuf-compiler \
    clang \
    make \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install \
      --bin linera \
      --locked \
      linera-service@0.15.6
