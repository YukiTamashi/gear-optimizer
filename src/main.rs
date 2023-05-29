use axum::{routing::get, Router};
use elasticsearch::{http::transport::Transport, Elasticsearch, IndexParts};
use serde_json::json;

static XIV_API: &str = "https://xivapi.com";

async fn get_item() -> String {
    let transport = Transport::single_node(XIV_API).unwrap();
    let client = Elasticsearch::new(transport);
    let request = client
        .index(IndexParts::IndexId("item", "1"))
        .body(json!(
            {
                "ID": 1613
            }
        ))
        .send()
        .await;
    if let Ok(response) = request {
        response.text().await.unwrap()
    } else {
        "request failed".to_owned()
    }
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(get_item));

    Ok(router.into())
}
