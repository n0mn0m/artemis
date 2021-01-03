use super::autofac::AutoFacModule;
use super::rocket_config::Settings;
use crate::routes::diagnostic;

pub fn launchpad(services: AutoFacModule, settings: Settings) -> rocket::Rocket {
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
                diagnostic::set_loglevel,
                diagnostic::system_time
            ],
        )
}
