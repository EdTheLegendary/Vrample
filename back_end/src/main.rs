mod in_memory;

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::get,
    Json, Router,
};
use std::net::SocketAddr;

use in_memory::{load_state, rest_router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .merge(rest_router())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
        )
        .with_state(load_state());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("stopped listening");
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: format!("Hello, World!"),
    })
}
