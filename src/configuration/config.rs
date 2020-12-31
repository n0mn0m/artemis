use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct IpRateLimiting {
    pub use_distributed_caching: bool,
    pub enable_endpoint_rate_limiting: bool,
    pub stack_blocked_requests: bool,
    pub real_ip_header: String,
    pub http_status_code: u16,
    pub ip_white_list: Vec<String>,
    pub endpoint_white_list: Vec<String>,
    pub general_rules: Vec<BTreeMap<String, String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct JwtToken {
    audience: String,
    issuer: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct IpRateLimitPolicies {
    pub ip_rules: Vec<Option<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Logging {
    pub log_level: LogLevel,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LogLevel {
    pub default: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Aws {
    pub region: String,
    pub secret_name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub application_name: String,
    pub application_id: String,
    pub urls: String,
    pub cors_allowed_domain: String,
    pub require_client_name_header_value: String,
    pub use_cloud_watch_metrics: String,
    pub logging: Logging,
    pub jwt_token: JwtToken,
    pub ip_rate_limiting: IpRateLimiting,
    pub ip_rate_limit_policies: IpRateLimitPolicies,
    pub aws: Aws,
}

impl Config {
    pub async fn new(path: String) -> Result<Config, io::Error> {
        let file = std::fs::File::open(path);
        match file {
            Err(e) => Err(e),
            Ok(f) => {
                let parsed: Config = serde_yaml::from_reader(f).unwrap();
                Ok(parsed)
            }
        }
    }
}
