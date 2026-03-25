FROM rust:1.86 as builder

WORKDIR /build

COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY src/ src/
COPY src/frontend/ src/frontend/
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/release/aiden /app/aiden
COPY --from=builder /build/src/frontend/index.html /app/index.html

RUN mkdir -p /app/docs

EXPOSE 8081

ENV RUST_LOG=info

CMD ["./aiden"]
