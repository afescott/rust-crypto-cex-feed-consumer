use axum::{body, http::Response, response::IntoResponse};
#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Order {
    pub created_time: String,
    pub symbol: String,
    pub avg_price: String,
    pub stop_loss: String,
    pub qty: String,
}
impl From<Order> for String {
    fn from(value: Order) -> Self {
        value.symbol
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BybitOrder {
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

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct KucoinOrder {
    id: String,
    symbol: String,
    r#type: String,
    price: String,
    size: String,
    #[serde(rename = "stopPrice")]
    stop_price: String,
}

impl From<KucoinOrder> for Order {
    fn from(value: KucoinOrder) -> Self {
        todo!()
    }
}

impl From<BybitOrder> for Order {
    fn from(value: BybitOrder) -> Self {
        Self {
            created_time: value.created_time,
            symbol: value.symbol,
            avg_price: value.avg_price,
            stop_loss: value.stop_loss,
            qty: value.qty,
        }
    }
}

impl IntoResponse for Order {
    fn into_response(self) -> axum::response::Response {
        Response::new(body::boxed(self.avg_price))
    }
}
