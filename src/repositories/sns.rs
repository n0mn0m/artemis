use rusoto_sns::SnsClient;

use rusoto_core::Region;

pub struct SnsHandle {
    pub client: SnsClient,
}

// use repositories::sns::SnsHandle;
// use rusoto_sns::{PublishInput, MessageAttributeValue, Sns};
// use serde_json::json;
// use std::collections::HashMap;
//
// let sns: SnsHandle = SnsHandle::new(&config.aws.region).await;
//
// let message_attributes: HashMap<String, MessageAttributeValue> =
// [("application".to_string(), MessageAttributeValue {
// binary_value: None,
// data_type: "String".to_string(),
// string_value: Option::from("notifications".to_string())
// }),
// ("secondary".to_string(), MessageAttributeValue {
// binary_value: None,
// data_type: "String".to_string(),
// string_value: Option::from("baz".to_string())
// }),
// ].iter().cloned().collect();
//
// let r = sns.client.publish(PublishInput {
// message: json!({"default": "Hello from Rust!"}).to_string(),
// message_attributes: Option::from(message_attributes),
// message_structure: Option::from("json".to_string()),
// phone_number: None,
// subject: None,
// target_arn: None,
// topic_arn: Option::from("arn:aws:sns:region:id:name".to_string())
// });
//
// dbg!(r.await.unwrap());

impl SnsHandle {
    pub async fn new(region: &str) -> SnsHandle {
        let r = match region {
            "us-east-1" => Region::UsEast1,
            "ap-southeast-1" => Region::ApSoutheast1,
            _ => panic!("Unsupported AWS Region.\nPlease correct region for the application to get the correct secret information.")
        };

        let client = SnsClient::new(r);

        SnsHandle { client }
    }
}
