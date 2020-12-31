use rustc_version_runtime::version;
use serde::Serialize;
use std::env;

use crate::configuration::config::Config;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Debug)]
pub struct ServiceInformation {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub region: String,
    pub launch_time: String,
    pub rustc_version: String,
}

impl ServiceInformation {
    pub fn new(config: &Config) -> ServiceInformation {
        ServiceInformation {
            name: config.application_name.to_string(),
            version: VERSION.to_string(),
            environment: match env::var_os("AwsEnvironment") {
                Some(val) => val.into_string().unwrap(),
                None => "local".to_string(),
            },
            rustc_version: version().to_string(),
            launch_time: chrono::offset::Utc::today().to_string(),
            region: config.aws.region.to_string(),
        }
    }

    pub fn info(&self) -> String {
        format!(
            "
##########################################################################


    Application Name: {}
    Application Version: {}
    Application Deployment Environment: {}
    Application Deployment Region: {}
    Application Launch Time: {}
    Rustc Compiler Version: {}


###########################################################################
",
            self.name,
            self.version,
            self.environment,
            self.region,
            self.launch_time,
            self.rustc_version
        )
    }
}
