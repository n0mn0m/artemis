use chrono::Datelike;

use std::io;

use super::autofac::{AutoFacModule, TodayWriter, TodayWriterParameters};
use super::service_information::ServiceInformation;
use crate::configuration::{config::Config, secrets::Secrets};

pub struct Settings(pub Config, pub Secrets, pub ServiceInformation);

pub async fn assemble_services() -> AutoFacModule {
    AutoFacModule::builder()
        .with_component_parameters::<TodayWriter>(TodayWriterParameters {
            today: chrono::offset::Utc::today().to_string(),
            year: chrono::offset::Utc::today().year(),
        })
        .build()
}

pub async fn assemble_settings() -> Result<Settings, io::Error> {
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
