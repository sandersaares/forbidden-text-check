use axum::{Router, http::StatusCode, response::IntoResponse, routing::post};
use forbidden_text_check::is_forbidden_text_static;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/check", post(check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 1234));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn check(body: String) -> impl IntoResponse {
    if is_forbidden_text_static(&body) {
        (StatusCode::OK, "true")
    } else {
        (StatusCode::OK, "false")
    }
}
