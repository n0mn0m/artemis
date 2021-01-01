# Latest rust version.
FROM rust as planner
ENV RUSTFLAGS="-Zinstrument-coverage"
ENV RUSTC_BOOTSTRAP=1
ENV LLVM_PROFILE_FILE="reports/llvm.profraw"
RUN rustup component add llvm-tools-preview
RUN cargo install grcov cargo-make

RUN wget https://packages.microsoft.com/config/ubuntu/18.04/packages-microsoft-prod.deb \
    -O packages-microsoft-prod.deb && \
    sudo dpkg -i packages-microsoft-prod.deb

RUN sudo apt-get update && \
    sudo apt-get install -y apt-transport-https && \
    sudo apt-get update && \
    sudo apt-get install -y dotnet-sdk-5.0

WORKDIR app
COPY . .

RUN cargo build --verbose
ENTRYPOINT cargo make tests && cargo make grcov && cargo make coverage
