# Set rust version running on debian buster slim image.
FROM rustlang/rust:nightly-buster-slim as builder
WORKDIR app
COPY . .
RUN cargo build --release --bin artemis

FROM rustlang/rust:nightly-buster-slim as runtime
WORKDIR app
COPY --from=builder /app/target/release/artemis /usr/local/bin
ENTRYPOINT ["/usr/local/bin/artemis"]
