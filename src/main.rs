use axum::{routing::get, Router};

async fn hello_world() -> String {
    let request = reqwest::get("https://xivapi.com/Item/1675").await;
    if let Ok(response) = request {
        response.text().await.unwrap()
    } else {
        "request failed".to_owned()
    }
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
