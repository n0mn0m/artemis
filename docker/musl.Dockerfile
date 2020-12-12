# Experiment, easy cross compilation opens up interesting opportunities for target architectures.
FROM rustembedded/cross:x86_64-unknown-linux-musl-0.2.1

COPY target/x86_64-unknown-linux-musl/release/ams_notifications /usr/local/bin

ENTRYPOINT ["/usr/local/bin/ams_notifications"]
