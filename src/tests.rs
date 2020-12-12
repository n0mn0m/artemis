use crate::{configure_services, rocket};
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn hello_world() {
    let client = Client::new(rocket(configure_services())).unwrap();
    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("Today is June 19, 2020".into())
    );
}
