use axum::{Router, http::StatusCode, response::IntoResponse, routing::post};
use forbidden_text_check::{into_variants, is_forbidden_text_static};
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
    // We check different variants of the input string, to create
    // a more realistic workload with some async+await activity.
    let variants = into_variants(body);

    let mut tasks = Vec::with_capacity(variants.len());
    for variant in variants {
        tasks.push(tokio::spawn(
            async move { is_forbidden_text_static(&variant) },
        ));
    }

    let results = futures::future::join_all(tasks).await;

    if results.into_iter().any(|r| r.unwrap()) {
        (StatusCode::OK, "true")
    } else {
        (StatusCode::OK, "false")
    }
}
