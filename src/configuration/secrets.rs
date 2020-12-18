use rusoto_core::Region;
use rusoto_secretsmanager::{GetSecretValueRequest, SecretsManager, SecretsManagerClient};

use serde::{Deserialize};

// TODO Change this to a nested yaml like setup to
// destructure and pass pieces around i.e. mongo, elastic, aws
#[derive(Deserialize, Debug, Clone)]
pub struct Secrets {
    pub environment: String,
    pub elasticsearch_endpoint: String,
    pub elasticsearch_username: String,
    pub elasticsearch_password: String,
    pub mongo_connection: String,
    pub mongo_username: String,
    pub mongo_password: String,
    pub memcached_host: String,
    pub aws_iam_key: String,
    pub aws_iam_secret: String,
    pub aws_jwt_access_key_id: String,
    pub aws_jwt_secret_key_id: String,
    pub aws_jwt_management_region: String,
    pub aws_jwt_management_bucket: String,
    pub aws_jwt_management_key: String,
}

impl Secrets {
    pub async fn new(name: &str, region: &str) -> Secrets {
        let r = match region {
            "us-east-1" => Region::UsEast1,
            "ap-southeast-1" => Region::ApSoutheast1,
            _ => panic!("Unsupported AWS Region.\nPlease correct region for the application to get the correct secret information.")
        };

        let secret_client = SecretsManagerClient::new(r);

        let r = GetSecretValueRequest {
            secret_id: name.to_string(),
            version_id: None,
            version_stage: None,
        };

        let s = secret_client.get_secret_value(r).await.unwrap();
        let v = s.secret_string.as_ref().unwrap().to_string();
        let parsed: Secrets = serde_json::from_str(v.as_str()).unwrap();
        parsed
    }
}
