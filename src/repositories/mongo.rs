use mongodb::{options::ClientOptions, options::Credential, Client};

use crate::configuration::config::Config;
use crate::configuration::secrets::Secrets;

pub struct MongoHandle {
    pub client: Client,
}

// let mongo = MongoClient::new(&config, &secrets).await;
// let db = mongo.client.database("foo");
//
// let collections = db.list_collection_names(None).await.unwrap() ;
//
// for c in collections {
//     println!("{}", c);
// }
//
impl MongoHandle {
    pub async fn new(config: &Config, secret: &Secrets) -> MongoHandle {
        // Using this instead of the builder to get a better hosts vec with Atlas.
        let mut options = ClientOptions::parse(secret.mongo_connection.as_str())
            .await
            .unwrap();
        options.app_name = Some(config.application_name.clone());
        options.credential = Some(
            Credential::builder()
                .username(secret.mongo_username.clone())
                .password(secret.mongo_password.clone())
                .build(),
        );

        dbg!(&options);
        let client = Client::with_options(options).unwrap();
        MongoHandle { client }
    }
}
