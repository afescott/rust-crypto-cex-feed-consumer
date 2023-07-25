use axum::{body, http::Response, response::IntoResponse};

use crate::repositories::{mem::Key, Provider, Repository};

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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
impl<T: Provider + Clone> Repository<Order>
    for crate::repositories::mem::StorageRepository<T, Order>
{
    type Provider = T;

    fn provider(&self) -> &Self::Provider {
        &self.provider
    }

    fn store_data(&self, results: Vec<Order>) {
        let result = self.state.lock().unwrap();

        for ele in results {
            let key = Key::create(ele.symbol.clone(), ele.created_time.clone());
            println!("{:?}", key);
            // if !result.contains_key(&key) {
            let mut result = result[&key].get(&ele);
            println!("{:?}", result);

            result.insert(result.unwrap());
            //if time is within one day, and symbol is the same add it to the hashset
            // }
        }
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
