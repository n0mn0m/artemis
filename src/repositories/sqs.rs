use rusoto_sqs::SqsClient;

use rusoto_core::Region;

pub struct SqsHandle {
    pub client: SqsClient,
}

// let sqs = SqsHandle::new(&config.aws.region).await;
// let m = sqs.client.receive_message(ReceiveMessageRequest{
// attribute_names: None,
// max_number_of_messages: None,
// message_attribute_names: None,
// queue_url: "https://sqs.region.amazonaws.com/id/name".to_string(),
// receive_request_attempt_id: None,
// visibility_timeout: None,
// wait_time_seconds: None
// }).await.unwrap();
//
// let messages = m.messages.unwrap();
// dbg!(messages);

impl SqsHandle {
    pub async fn new(region: &str) -> SqsHandle {
        let r = match region {
            "us-east-1" => Region::UsEast1,
            "ap-southeast-1" => Region::ApSoutheast1,
            _ => panic!("Unsupported AWS Region.\nPlease correct region for the application to get the correct secret information.")
        };

        let client = SqsClient::new(r);

        SqsHandle { client }
    }
}
