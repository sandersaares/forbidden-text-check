use axum::{Router, http::StatusCode, response::IntoResponse, routing::post};
use illegal_numbers_check::ILLEGAL_NUMBERS;
use std::net::SocketAddr;
use tokio::spawn;

#[tokio::main]
async fn main() {
    // Pre-warm the data set so the first request does not get penalized.
    _ = ILLEGAL_NUMBERS.len();
    
    spawn(async move {
        let app = Router::new().route("/check", post(check_number));

        let addr = SocketAddr::from(([0, 0, 0, 0], 1234));
        println!("Server running on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    })
    .await
    .unwrap();
}

async fn check_number(body: String) -> impl IntoResponse {
    let contains_illegal = ILLEGAL_NUMBERS.iter().any(|num| body.contains(num));

    if contains_illegal {
        (StatusCode::OK, "true")
    } else {
        (StatusCode::OK, "false")
    }
}
