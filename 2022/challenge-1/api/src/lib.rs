use serde_json::json;
use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let result = json!({
        "message": "Hello, world!"
    });

    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(result.to_string())?)
}
