# Set rust version for compilation running in a distroless container.
# https://github.com/GoogleContainerTools/distroless
# If you need a debug shell https://github.com/GoogleContainerTools/distroless#debug-images
FROM rust:1.48 as planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.48 as cacher
WORKDIR app
RUN rustup default nightly
RUN rustup update
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.48 as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin ams_notifications

FROM gcr.io/distroless/cc
WORKDIR app
COPY --from=builder /app/target/release/ams_notifications /
ENTRYPOINT ["/ams_notifications"]
