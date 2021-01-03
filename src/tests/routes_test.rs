use chrono::Datelike;
use rocket::http::Status;
use rocket::local::Client;
use tokio_test::block_on;

use crate::routes;
use crate::utils;
use crate::utils::rocket_config::Settings;
use crate::utils::service_information::ServiceInformation;

use super::common;

#[test]
fn system_time() {
    let sv = block_on(utils::rocket_config::assemble_services());
    let c = common::get_test_config();
    let sc = common::get_test_secrets();
    let si = ServiceInformation::new(&c);

    let st = Settings(c, sc, si);

    let client =
        Client::new(utils::rocket_launch::launchpad(sv, st)).expect("valid rocket instance");
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
    let sv = block_on(utils::rocket_config::assemble_services());
    let c = common::get_test_config();
    let sc = common::get_test_secrets();
    let si = ServiceInformation::new(&c);

    let st = Settings(c, sc, si);

    let client =
        Client::new(utils::rocket_launch::launchpad(sv, st)).expect("valid rocket instance");
    let response = client.get("/diagnostic/healthcheck").dispatch();

    assert_eq!(response.status(), Status::NoContent);
}
