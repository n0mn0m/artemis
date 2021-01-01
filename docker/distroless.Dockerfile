# Set rust version for compilation running in a distroless container.
# https://github.com/GoogleContainerTools/distroless
# If you need a debug shell https://github.com/GoogleContainerTools/distroless#debug-images
FROM rustlang/rust:nightly as builder
WORKDIR app
COPY . .
RUN cargo build --release --bin artemis

FROM gcr.io/distroless/cc
WORKDIR app
COPY --from=builder /app/target/release/artemis /
ENTRYPOINT ["/artemis"]
