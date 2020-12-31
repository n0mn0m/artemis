use rusoto_ses::SesClient;

use rusoto_core::Region;

// use repositories::ses::SesHandle;
// let s = SesHandle::new(config.aws.region.as_str()).await;
// let x = s.client.send_email(SendEmailRequest{
//     configuration_set_name: None,
//     destination: Destination{
//         bcc_addresses: None,
//         cc_addresses: None,
//         to_addresses: Some(vec!["somebody@example.com".to_string()])
//     },
//     message: Message{
//     body: Body{
//         html: None,
//         text: Some(Content{
//             charset: None,
//             data: "Hello World".to_string()
//         })
//     },
//     subject: Content{
//         charset: None,
//         data: "Hi from SES Ops".to_string()
//         }
//     },
//     reply_to_addresses: Some(vec!["foo@bar.awsapps.com".to_string()]),
//     return_path: None,
//     return_path_arn: None,
//     source: "no-reply@bar.awsapps.com".to_string(),
//     source_arn: Some("arn:aws:ses:region:id:identity/foo@bar.awsapps.com".to_string()),
//     tags: None
// }).await.unwrap();

pub struct SesHandle {
    pub client: SesClient,
}

impl SesHandle {
    pub async fn new(region: &str) -> SesHandle {
        let r = match region {
            "us-east-1" => Region::UsEast1,
            "ap-southeast-1" => Region::ApSoutheast1,
            _ => panic!("Unsupported AWS Region.\nPlease correct region for the application to get the correct secret information.")
        };

        let client = SesClient::new(r);

        SesHandle { client }
    }
}
