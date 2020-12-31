#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use shaku_rocket::Inject;

use std::io;
use tokio::task;
use tracing::info;

mod autofac;
mod configuration;
use configuration::{config::Config, secrets::Secrets};

mod repositories;
mod telemetry;

mod tests;
use crate::autofac::{AutoFacModule, IDateWriter, TodayWriter, TodayWriterParameters};

mod utils;
use utils::service_information::ApplicationInformation;

#[get("/")]
fn index(writer: Inject<AutoFacModule, dyn IDateWriter>) -> String {
    writer.write_date();
    writer.get_date()
}

fn configure_services() -> autofac::AutoFacModule {
    AutoFacModule::builder()
        .with_component_parameters::<TodayWriter>(TodayWriterParameters {
            today: "June 19".to_string(),
            year: 2020,
        })
        .build()
}

fn rocket(services: autofac::AutoFacModule) -> rocket::Rocket {
    rocket::ignite().manage(services).mount("/", routes![index])
}

#[tokio::main]
async fn main() {
    let threadpool_future =
        task::spawn_blocking(|| Config::new("./src/app_config.yml".to_string()));

    let config = threadpool_future.await.unwrap();
    let _secrets = Secrets::new(&config.aws.secret_name, &config.aws.region).await;

    let app_info = ApplicationInformation::new(&config);

    let subscriber = tracing_subscriber::fmt().with_writer(io::stderr).finish();
    tracing::subscriber::with_default(subscriber, || {
        info!("{}", &app_info.banner_message());
    });

    rocket(configure_services()).launch();
}
