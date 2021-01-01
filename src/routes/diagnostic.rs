use rocket::http::Status;
use rocket::State;
use shaku_rocket::Inject;

use crate::configuration::config::Config;
use crate::utils::autofac::{AutoFacModule, IDateWriter};
use crate::utils::service_information::ServiceInformation;

#[get("/healthcheck")]
pub fn healthcheck() -> Status {
    Status::NoContent
}

#[get("/about")]
pub fn about(s: State<ServiceInformation>) -> String {
    s.info()
}

#[get("/systemtime")]
pub fn system_time(writer: Inject<AutoFacModule, dyn IDateWriter>) -> String {
    writer.write_date();
    writer.get_date()
}

// TODO: Get log level from active log object as it may mutate
#[get("/loglevel")]
pub fn current_loglevel(c: State<Config>) -> String {
    c.logging.log_level.default.clone()
}

// TODO: Adjust current log level
// TODO: Add jwt auth
#[post("/loglevel/<level>")]
pub fn set_loglevel(level: String) -> String {
    level
}
