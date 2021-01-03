#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use tracing::info;

use std::io;

mod configuration;
mod repositories;
mod routes;
mod telemetry;
mod tests;
mod utils;

use crate::utils::rocket_config::{assemble_services, assemble_settings};
use crate::utils::rocket_launch::launchpad;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt().with_writer(io::stderr).finish();
    tracing::subscriber::with_default(subscriber, || {
        info!("Hello from tracing");
    });

    let settings = assemble_settings().await.unwrap();
    let services = assemble_services().await;

    launchpad(services, settings).launch();
}
