# Latest rust version.
FROM rust as planner
ENV RUSTFLAGS="-Zinstrument-coverage"
ENV RUSTC_BOOTSTRAP=1
ENV LLVM_PROFILE_FILE="reports/llvm.profraw"
RUN rustup component add llvm-tools-preview
RUN cargo install grcov cargo-make

WORKDIR app
COPY . .

RUN cargo build --verbose
ENTRYPOINT cargo make tests && cargo make grcov
