#[cfg(test)]
use crate::*;
#[cfg(test)]
use rocket::http::Status;
#[cfg(test)]
use rocket::local::Client;
#[cfg(test)]
use tokio_test::block_on;

use crate::configuration::config::{
    Aws, IpRateLimitPolicies, IpRateLimiting, JwtToken, LogLevel, Logging,
};

fn get_test_config() -> Config {
    Config {
        application_name: "test-app".to_string(),
        application_id: "234234-test-app".to_string(),
        urls: "test-app.foo.rs".to_string(),
        cors_allowed_domain: "".to_string(),
        require_client_name_header_value: "".to_string(),
        use_cloud_watch_metrics: "".to_string(),
        logging: Logging {
            log_level: LogLevel {
                default: "".to_string(),
            },
        },
        jwt_token: JwtToken {
            audience: "".to_string(),
            issuer: "".to_string(),
        },
        ip_rate_limiting: IpRateLimiting {
            use_distributed_caching: false,
            enable_endpoint_rate_limiting: false,
            stack_blocked_requests: false,
            real_ip_header: "".to_string(),
            http_status_code: 0,
            ip_white_list: vec![],
            endpoint_white_list: vec![],
            general_rules: vec![],
        },
        ip_rate_limit_policies: IpRateLimitPolicies { ip_rules: vec![] },
        aws: Aws {
            region: "us-east-1".to_string(),
            secret_name: "test-foo".to_string(),
        },
    }
}

fn get_test_secrets() -> Secrets {
    Secrets {
        environment: "test".to_string(),
        elasticsearch_endpoint: "".to_string(),
        elasticsearch_username: "".to_string(),
        elasticsearch_password: "".to_string(),
        mongo_connection: "".to_string(),
        mongo_username: "".to_string(),
        mongo_password: "".to_string(),
        memcached_host: "".to_string(),
        aws_iam_key: "".to_string(),
        aws_iam_secret: "".to_string(),
        aws_jwt_access_key_id: "".to_string(),
        aws_jwt_secret_key_id: "".to_string(),
        aws_jwt_management_region: "".to_string(),
        aws_jwt_management_bucket: "".to_string(),
        aws_jwt_management_key: "".to_string(),
    }
}

#[test]
fn system_time() {
    let sv = block_on(configure_services());
    let c = get_test_config();
    let sc = get_test_secrets();
    let si = ServiceInformation::new(&c);

    let st = Settings(c, sc, si);

    let client = Client::new(launchpad(sv, st)).expect("valid rocket instance");
    let mut response = client.get("/diagnostic/systemtime").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some(format!(
            "Today is {}, {}",
            chrono::offset::Utc::today().to_string(),
            chrono::offset::Utc::today().year()
        ))
    );
}

#[test]
fn healthcheck() {
    let sv = block_on(configure_services());
    let c = get_test_config();
    let sc = get_test_secrets();
    let si = ServiceInformation::new(&c);

    let st = Settings(c, sc, si);

    let client = Client::new(launchpad(sv, st)).expect("valid rocket instance");
    let mut response = client.get("/diagnostic/healthcheck").dispatch();

    assert_eq!(response.status(), Status::NoContent);
}
