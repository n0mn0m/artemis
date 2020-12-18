use elasticsearch::{Elasticsearch, BulkParts, auth::Credentials, http::transport::{SingleNodeConnectionPool, TransportBuilder}, http::Url};

use chrono;
use chrono::Datelike;

use elasticsearch::http::StatusCode;

use crate::telemetry::message::Message;

#[derive(Debug, Clone)]
pub struct ElasticsearchClient {
    index: String,
    pub client: Elasticsearch
}

fn index_format() -> String {
    let index_name = "operations".to_string();
    let n = chrono::offset::Utc::today();
    let m = n.month().to_string();
    let y = n.year().to_string();
    let index_format = format!("{}-{}.{}", index_name, y, m);
    index_format
}

impl ElasticsearchClient {

    pub fn new(url: String, user: String, pass: String) -> ElasticsearchClient
    {
        let conn_pool = SingleNodeConnectionPool::new(Url::parse(url.as_str()).unwrap());
        let credentials = Credentials::Basic(user, pass);
        let transport = TransportBuilder::new(conn_pool).auth(credentials).build().unwrap();
        let client = Elasticsearch::new(transport);
        let index = index_format();

        ElasticsearchClient { client, index }
    }

    pub async fn bulk_write(&self, messages: Vec<Message>) -> StatusCode {
        let json_messages: Vec<String> = messages.iter().map(|m| serde_json::to_string(&m).unwrap()).collect();
        // dbg!(&json_messages);

        let index = &self.index;
        // dbg!(&index);

        let response = self.client
            .bulk(BulkParts::Index(index))
            .body(json_messages)
            .error_trace(true)
            .send()
        .await.unwrap();

        // dbg!(&response);
        response.status_code()
    }
}

