use serde::Deserialize;
use serde_json::json;
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.post("/distance-latlong", calculate_distance);
    router.any("*", not_found);
    Ok(router.handle(req))
}

fn not_found(_req: Request, _p: Params) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::new(404, "Not Found"))
}

#[derive(Deserialize)]
struct Destination {
    lat: f64,
    long: f64,
}

#[derive(Deserialize)]
struct RequestModel {
    #[serde(rename = "d1")]
    destination_1: Destination,
    #[serde(rename = "d2")]
    destination_2: Destination,
}
fn calculate_distance(req: Request, _p: Params) -> anyhow::Result<impl IntoResponse> {
    let body = req.body();
    let model = serde_json::from_slice::<RequestModel>(body)?;

    let start = haversine::Location {
        latitude: model.destination_1.lat,
        longitude: model.destination_1.long,
    };
    let end = haversine::Location {
        latitude: model.destination_2.lat,
        longitude: model.destination_2.long,
    };

    let distance_in_miles = haversine::distance(start, end, haversine::Units::Miles);
    let distance_in_nautical_miles = distance_in_miles / 1.151_f64;
    
    let distance_in_nautical_miles = (distance_in_nautical_miles * 10.0).round() / 10.0;
    
    let response = json!({
        "distance":  distance_in_nautical_miles
    });
    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(response.to_string())?)
}
