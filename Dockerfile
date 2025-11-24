FROM rust:1.86-slim-bookworm AS builder

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


FROM debian:bookworm-slim AS final
COPY --from=builder /usr/local/cargo/bin/linera /usr/local/linera/bin/linera
ENTRYPOINT ["/usr/local/linera/bin/linera"]
