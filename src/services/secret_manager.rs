extern crate rusoto_core;
use rusoto_secretsmanager::{
    GetSecretValueRequest, GetSecretValueResponse, ListSecretsRequest, ListSecretsResponse,
    SecretsManager, SecretsManagerClient,
};

extern crate futures;
use futures::executor::block_on;

use std::collections::HashMap;
use std::default::Default;

pub struct SecretStore {
    pub secrets: Vec<String>,
    pub shelf: HashMap<String, String>,
}

impl SecretStore {
    pub fn new(region: rusoto_core::Region) -> SecretStore {
        let secret_client = SecretsManagerClient::new(region);

        let secrets: ListSecretsResponse =
            block_on(secret_client.list_secrets(ListSecretsRequest::default())).unwrap();

        let secret_list = secrets.secret_list.unwrap();

        let s: Vec<GetSecretValueResponse> = secret_list
            .iter()
            .map(|k| {
                let r = GetSecretValueRequest {
                    secret_id: k.name.as_ref().unwrap().to_string(),
                    version_id: None,
                    version_stage: None,
                };
                let v = block_on(secret_client.get_secret_value(r)).unwrap();
                v
            })
            .collect();

        let mut k: HashMap<String, String> = HashMap::new();
        for v in s.iter() {
            k.insert(
                v.name.as_ref().unwrap().to_string(),
                v.secret_string.as_ref().unwrap().to_string(),
            );
        }

        let names = s
            .iter()
            .map(|v| v.name.as_ref().unwrap().to_string())
            .collect();
        SecretStore {
            secrets: names,
            shelf: k,
        }
    }
}
