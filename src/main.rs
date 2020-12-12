#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod autofac;
use crate::autofac::{AutoFacModule, IDateWriter, TodayWriter, TodayWriterParameters};
use shaku_rocket::Inject;

mod tests;

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

fn main() {
    rocket(configure_services()).launch();
}
