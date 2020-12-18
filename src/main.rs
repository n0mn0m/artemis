#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use shaku_rocket::Inject;

use tokio::task;
use std::io;
use tracing::{info};

mod autofac;
mod configuration;
use configuration::{config::Config, secrets::Secrets};

mod tests;
use crate::autofac::{AutoFacModule, IDateWriter, TodayWriter, TodayWriterParameters};

mod telemetry;
use telemetry::elastic::ElasticsearchClient;
use telemetry::message::Message;
use elasticsearch::http::StatusCode;

mod repositories;
use repositories::service_information::ApplicationInformation;


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

async fn write_to_elastic(secrets: Secrets) -> StatusCode {
    // TODO replace with destructured struct
    let url = secrets.elasticsearch_endpoint.clone();
    let user = secrets.elasticsearch_username.clone();
    let pass = secrets.elasticsearch_password.clone();

    let client = ElasticsearchClient::new(url, user, pass);
    dbg!(&client);

    let messages = vec![
        Message::new("Startup".to_string()),
        Message::new("Send".to_string())
    ];

    client.bulk_write(messages).await
}

#[tokio::main]
async fn main() {
    let threadpool_future = task::spawn_blocking(|| {
        Config::new("./src/app_config.yml".to_string())
    });

    let config = threadpool_future.await.unwrap();
    let _secrets = Secrets::new(&config.aws.secret_name, &config.aws.region).await;

    let app_info = ApplicationInformation::new(&config);


    let subscriber = tracing_subscriber::fmt().with_writer(io::stderr).finish();
    tracing::subscriber::with_default(subscriber, || {
        info!("{}", &app_info.banner_message());
    });

    // let t = write_to_elastic(secrets).await;
    // println!("{}", t);

    // rocket(configure_services()).launch();
}
