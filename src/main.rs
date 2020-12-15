#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use shaku_rocket::Inject;

use rusoto_core::Region;
use tokio::task;

mod autofac;
mod services;
mod tests;
use crate::autofac::{AutoFacModule, IDateWriter, TodayWriter, TodayWriterParameters};

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
        task::spawn_blocking(|| services::secret_manager::SecretStore::new(Region::UsEast1));
    let _secrets = threadpool_future.await.unwrap();
    // dbg!("{}", _secrets.secrets);
    // dbg!("{}", _secrets.shelf);

    rocket(configure_services()).launch();
}
