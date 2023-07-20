use axum::{body, http::Response, response::IntoResponse};
use kucoin_rs::serde_json::{self, Value};
use serde::Deserialize;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Order {
    #[serde(rename = "createdTime")]
    pub created_time: String,
    pub symbol: String,
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    #[serde(rename = "stopLoss")]
    pub stop_loss: String,
    #[serde(rename = "cumExecQty")]
    pub qty: String,
}

impl IntoResponse for Order {
    fn into_response(self) -> axum::response::Response {
        Response::new(body::boxed(self.avg_price))
    }
}

// impl From<Value> for Order {
//     fn from(value: Value) -> Self {
//         if let Some(value) = value["result"]["list"].as_array() {
//             println!("{:?}", value);
//             for ele in value {
//                 kucoin_rs::serde_json::from_value(ele.clone()).unwrap()
//             }
//         }
//     }
// }

// #[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
// pub struct OrderModels {
//     pub avgPrice: String,
//     pub stopLoss: String,
//     pub qty: String,
// }
