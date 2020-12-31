#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use shaku_rocket::Inject;
use std::io;
use tracing::info;

mod autofac;
mod configuration;
mod repositories;
mod routes;
mod telemetry;
mod tests;
mod utils;

use autofac::{AutoFacModule, IDateWriter, TodayWriter, TodayWriterParameters};
use configuration::{config::Config, secrets::Secrets};
use routes::diagnostic;
use utils::service_information::ServiceInformation;

#[get("/")]
fn index(writer: Inject<AutoFacModule, dyn IDateWriter>) -> String {
    writer.write_date();
    writer.get_date()
}

async fn configure_services() -> autofac::AutoFacModule {
    AutoFacModule::builder()
        .with_component_parameters::<TodayWriter>(TodayWriterParameters {
            today: chrono::offset::Utc::today().to_string(),
            year: 2020,
        })
        .build()
}

struct Settings(Config, Secrets, ServiceInformation);

async fn crawler() -> Result<Settings, io::Error> {
    let config = Config::new("./src/app_config.yml".to_string()).await;
    match config {
        Err(e) => Err(e),
        Ok(c) => {
            let s = Secrets::new(&c.aws.secret_name, &c.aws.region).await;
            let i = ServiceInformation::new(&c);
            Ok(Settings(c, s, i))
        }
    }
}

fn launchpad(services: autofac::AutoFacModule, settings: Settings) -> rocket::Rocket {
    rocket::ignite()
        .manage(services)
        .manage(settings.0)
        .manage(settings.1)
        .manage(settings.2)
        .mount(
            "/diagnostic",
            routes![
                diagnostic::healthcheck,
                diagnostic::about,
                diagnostic::current_loglevel,
                diagnostic::set_loglevel
            ],
        )
        .mount("/", routes![index])
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt().with_writer(io::stderr).finish();
    tracing::subscriber::with_default(subscriber, || {
        info!("Hello from tracing");
    });

    let settings = crawler().await.unwrap();
    let services = configure_services().await;

    launchpad(services, settings).launch();
}
