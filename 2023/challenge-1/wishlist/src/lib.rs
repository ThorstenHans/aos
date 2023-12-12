use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Json, Params, Request, Router};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

#[derive(Debug, Serialize, Deserialize)]
struct Wishlist {
    value: String,
}

#[http_component]
fn handle_wishlist(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut r = Router::default();
    r.post("/data", handle_post);
    r.get("/data", handle_get);
    Ok(r.handle(req))
}

fn handle_post(
    req: http::Request<Json<Wishlist>>,
    _params: Params,
) -> anyhow::Result<impl IntoResponse> {
    let Some(key) = req.uri().query() else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body("Not found")?);
    };
    let store = Store::open_default()?;
    store.set_json(key, &req.body().0)?;
    Ok(http::Response::builder()
        .status(http::StatusCode::CREATED)
        .header("Content-Type", "text/plain")
        .body("Wishlist stored")?)
}

fn handle_get(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let key = req.query();
    if key.is_empty() {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body("Not found".to_string())?);
    }

    let store = Store::open_default()?;
    match store.get_json::<Wishlist>(key)? {
        Some(w) => {
            let response_body = serde_json::to_string(&w)?;
            Ok(http::Response::builder()
                .status(http::StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(response_body)?)
        }
        None => Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body("Not found".to_string())?),
    }
}
