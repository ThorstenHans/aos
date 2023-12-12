use http::Response;
use serde::Deserialize;
use serde_json::json;
use spin_sdk::http::{IntoResponse, Params, Request, Router};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.post("/lowercase", to_lowercase);
    router.get("/hello", say_hello);
    router.get("/hello/:name", say_hello);
    router.any("*", not_found);
    Ok(router.handle(req))
}

fn not_found(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(404)
        .header("Content-Type", "application/json")
        .body("Not found")?)
}

#[derive(Deserialize)]
struct RequestModel {
    value: String,
}
fn to_lowercase(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let model: RequestModel = serde_json::from_slice(req.body())?;
    let result = json!({
        "message": model.value.to_lowercase()
    });
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(result.to_string())?)
}

fn say_hello(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let name = params
        .get("name")
        .unwrap_or("World")
        .to_lowercase();

    let result = json!({
        "message": format!("Hello, {}!", name)
    });
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(result.to_string())?)
}
