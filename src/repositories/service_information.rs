use crate::configuration::config::Config;
use std::env;

use serde::Serialize;

use rustc_version_runtime::version;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Debug)]
pub struct ApplicationInformation {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub region: String,
    pub launch_time: String,
    pub rustc_version: String,
}

impl ApplicationInformation {
    pub fn new(config: &Config) -> ApplicationInformation {
        ApplicationInformation {
            name: config.application_name.clone().to_string(),
            version: VERSION.to_string(),
            environment: match env::var_os("AwsEnvironment") {
                Some(val) => val.into_string().unwrap(),
                None => "local".to_string()
            },
            rustc_version: version().to_string(),
            launch_time: chrono::offset::Utc::today().to_string(),
            region: config.aws.region.clone().to_string()
        }
    }

    pub fn banner_message(&self) -> String {
       format!("
##########################################################################


    Application Name: {}
    Application Version: {}
    Application Deployment Environment: {}
    Application Deployment Region: {}
    Application Launch Time: {}
    Rustc Compiler Version: {}


###########################################################################
", self.name, self.version, self.environment, self.region, self.launch_time, self.rustc_version)
    }
}
