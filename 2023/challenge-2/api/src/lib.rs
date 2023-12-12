use serde::Deserialize;
use serde_json::json;
use spin_sdk::http::{IntoResponse, Params, Request, Router};
use spin_sdk::http_component;

#[derive(Debug, Deserialize)]
struct RequestModel {
    pub kids: Vec<u32>,
    pub weight: Vec<u32>,
    pub capacity: u32,
}

struct Delivery {
    pub number_of_kids: u32,
    pub weight_of_presents: u32,
}

impl From<&RequestModel> for Vec<Delivery> {
    fn from(model: &RequestModel) -> Self {
        let mut stops: Vec<Delivery> = model
            .kids
            .iter()
            .zip(model.weight.iter())
            .map(|(kids, weight)| Delivery {
                number_of_kids: *kids,
                weight_of_presents: *weight,
            })
            .collect();
        stops.sort_by(|a, b| b.number_of_kids.cmp(&a.number_of_kids));
        stops
    }
}

#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.post("/", solve_challenge);
    Ok(router.handle(req))
}

fn solve_challenge(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let p = req.body();
    let api_model = serde_json::from_slice::<RequestModel>(p)?;
    let model: Vec<Delivery> = (&api_model).into();

    let reached_kids = solve(api_model.capacity, &model);
    let result = json!({
        "kids": reached_kids
    });

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(result.to_string())?)
}

fn solve(capacity: u32, model: &Vec<Delivery>) -> u32 {
    let mut remaining_capacity = capacity;
    let mut reached_kids = 0;

    for stop in model {
        if remaining_capacity >= stop.weight_of_presents {
            remaining_capacity -= stop.weight_of_presents;
            reached_kids += stop.number_of_kids;
        } else {
            break;
        }
    }
    reached_kids
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1200, vec![10, 40, 50, 60, 30, 20, 70], vec![230, 450, 230, 430, 120, 320, 450], 180)]
    #[case(120, vec![1, 4, 5, 6, 3, 2, 7], vec![23, 45, 23, 43, 12, 32, 45], 18)]
    #[case(50, vec![5, 1, 4, 3], vec![12, 20, 23, 10], 12)]
    fn solve_challenge(
        #[case] capacity: u32,
        #[case] kids: Vec<u32>,
        #[case] weight: Vec<u32>,
        #[case] expected: u32,
    ) {
        let model = &RequestModel {
            kids,
            weight,
            capacity,
        };
        let actual = solve(model.capacity, &model.into());
        assert_eq!(actual, expected);
    }
}
