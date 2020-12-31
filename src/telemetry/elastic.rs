use chrono::Datelike;
use elasticsearch::{
    auth::Credentials,
    http::request::JsonBody,
    http::response::Response,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    http::Url,
    BulkParts, Elasticsearch,
};
use serde_json::json;

use super::message::Message;

#[derive(Debug, Clone)]
pub struct ElasticsearchHandle {
    index: String,
    pub client: Elasticsearch,
}

fn log_index() -> String {
    let index_name = "ops".to_string();
    let n = chrono::offset::Utc::today();
    let m = n.month().to_string();
    let y = n.year().to_string();
    let index_format = format!("{}-{}.{}", index_name, y, m);
    index_format
}

// let es = telemetry::elastic::ElasticsearchHandle::new(url, user, pass, index);
// let m = telemetry::message::Message::new("Hello".to_string());
// let esm = es.client.create(CreateParts::IndexId("test", "12292020")).body(json!({
//         "id": 12292020,
//         "message": "hello world"
//         })).error_trace(true).send().await.unwrap();
//
// dbg!(esm);
impl ElasticsearchHandle {
    pub fn new(
        url: String,
        user: String,
        pass: String,
        index: Option<String>,
    ) -> ElasticsearchHandle {
        let conn_pool = SingleNodeConnectionPool::new(Url::parse(url.as_str()).unwrap());
        let credentials = Credentials::Basic(user, pass);
        let transport = TransportBuilder::new(conn_pool)
            .auth(credentials)
            .build()
            .unwrap();
        let client = Elasticsearch::new(transport);
        let index = match index {
            Some(i) => i,
            None => log_index(),
        };

        ElasticsearchHandle { client, index }
    }

    // let es = telemetry::elastic::ElasticsearchHandle::new(url, user, pass, None);
    //
    // let messages = vec![
    //     Message::new("Startup".to_string()),
    //     Message::new("Send".to_string())
    // ];
    //
    // let esm = es.log(messages).await;
    // dbg!(esm);
    pub async fn log(&self, messages: Vec<Message>) -> Response {
        let mut body: Vec<JsonBody<_>> = Vec::new();

        // create our index op off the id in our messages.
        for m in messages {
            body.push(json!({"index": {"_id": m.id.clone()}}).into());
            body.push(json!(m).into());
        }

        let response = self
            .client
            .bulk(BulkParts::Index(self.index.as_str()))
            .body(body)
            .error_trace(true)
            .send()
            .await
            .unwrap();

        response
    }
}
