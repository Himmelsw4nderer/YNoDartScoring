FROM rust:1.92 AS frontend_builder
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown
WORKDIR /frontend
COPY frontend/Cargo.toml frontend/Cargo.lock ./
COPY frontend/src ./src
COPY frontend/index.html ./index.html
RUN trunk build --release

FROM rust:1.92 AS backend_builder
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY backend .
RUN cargo fetch
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=backend_builder /app/target/release/YNoDartScoringBackend /usr/local/bin/
COPY --from=backend_builder /app/Rocket.toml /app/Rocket.toml
COPY --from=frontend_builder /frontend/dist /app/static
EXPOSE 8000
CMD ["YNoDartScoringBackend"]
