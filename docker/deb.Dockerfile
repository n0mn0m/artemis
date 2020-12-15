# Set rust version running on debian buster slim image.
FROM rust:1.48-slim-buster as planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.48-slim-buster as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.48-slim-buster as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin artemis

FROM rust:1.48-slim-buster as runtime
WORKDIR app
COPY --from=builder /app/target/release/artemis /usr/local/bin
ENTRYPOINT ["/usr/local/bin/artemis"]
