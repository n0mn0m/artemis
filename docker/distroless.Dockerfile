# Set rust version for compilation running in a distroless container.
# https://github.com/GoogleContainerTools/distroless
# If you need a debug shell https://github.com/GoogleContainerTools/distroless#debug-images
FROM rustlang/rust:nightly as planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rustlang/rust:nightly as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rustlang/rust:nightly as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin artemis

FROM gcr.io/distroless/cc
WORKDIR app
COPY --from=builder /app/target/release/artemis /
ENTRYPOINT ["/artemis"]
