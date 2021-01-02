# Overview

|distro|status|docker|
|:---|:---:|---:|
|disrtoless|[![Build Status](http://teamcity.burningdaylight.io/app/rest/builds/buildType:(id:artemis_primary)/statusIcon)](http://teamcity.burningdaylight.io/viewType.html?buildTypeId=artemis_primary&guest=1)|[![Docker Image](https://img.shields.io/docker/pulls/n0mn0m/artemis)](https://hub.docker.com/repository/docker/n0mn0m/artemis)|
|musl|[![Build Status](http://teamcity.burningdaylight.io/app/rest/builds/buildType:(id:artemis_musl)/statusIcon)](http://teamcity.burningdaylight.io/viewType.html?buildTypeId=artemis_musl&guest=1)|


Proof of concept Rocket project showing various services (tracing,
secrets, etc) managed and injected via dependency injection.

## Getting Started

This project uses [Rust](https://www.rust-lang.org/tools/install).

This project makes use of [`cargo make`](https://github.com/sagiegurari/cargo-make)
to provide cross platform dev and ci commands. To get started run
`cargo make` and the default command will get you going.

From there checkout `Makefile.toml` and continue into `docs` or `src`.

## Development

Custom build and development commands go in the root `Makefile.toml`.
When adding a command it's worth checking if it may already exist as
part of the default commands provided by [`cargo make`](https://github.com/sagiegurari/cargo-make/tree/master/src/lib/descriptor/makefiles).
The default command is setup to run all services required for testing,
format code, run clippy and execute test. `cargo make watch` will do
this as you make changes to the project.
