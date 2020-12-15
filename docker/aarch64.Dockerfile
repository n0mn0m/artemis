# Experiment, easy cross compilation opens up interesting opportunities for target architectures and platforms
# i.e. Graviton, BSD, etc.
FROM rustembedded/cross:aarch64-unknown-linux-gnu-0.2.1

COPY ./target/aarch64-unknown-linux-gnu/release/artemis /usr/local/bin

ENTRYPOINT ["/usr/local/bin/artemis"]
