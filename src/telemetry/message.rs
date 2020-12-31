use chrono;

use serde;
use serde::{Deserialize, Serialize};

use rand;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Message {
    pub id: String,
    pub applicationName: String,
    pub version: String,
    pub message: String,
}

impl Message {
    pub fn new(message: String) -> Message {
        Message {
            id: generate_id(),
            applicationName: "Foo.Bar.Baz".to_string(),
            version: chrono::offset::Utc::today().to_string(),
            message,
        }
    }
}

fn generate_id() -> String {
    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    rand_string
}
