# Overview

A lot has been written about rust compile times and docker. Rust has
a lot of tricks up it's sleeve for compiling, but for our project
we are using `cargo chef` and some simple docker builds to get started.

As new material of practices become important they should be added to
this doc.

## Example Docker Files

- [Rust Distroless](https://github.com/GoogleContainerTools/distroless/blob/master/examples/rust/Dockerfile)
- [Rust Playground Dockerfile](https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Dockerfile)
- [Rust Web Deb/Alpine](https://github.com/zupzup/rust-docker-web)

## Rust Docker Tools

- [Chef](https://github.com/LukeMathWalker/cargo-chef)
- [Cross](https://github.com/rust-embedded/cross)
- [Distroless](https://github.com/GoogleContainerTools/distroless)

## Git Tickets of Interest

- [cargo build --dependencies-only](https://github.com/rust-lang/cargo/issues/2644)
