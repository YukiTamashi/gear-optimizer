use axum::{routing::get, Router};

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(a));

    Ok(router.into())
}

async fn a<'a>() -> &'a str{
    "a"
}