use serde::de::DeserializeOwned;
use tokio::join;

use crate::models::order::Order;

pub mod bybit;
pub mod kucoin;
pub mod mem;

#[async_trait::async_trait]
pub trait Provider: Send + Sync + 'static {
    //could this provider retrieve selection & event data? Should 2 api's exist? One for selection and event
    /// Returns `EventId` list
    fn new() -> Self;

    async fn get_user_info<T: DeserializeOwned + core::fmt::Debug>(
        &self,
        request: RequestType,
    ) -> Result<Vec<T>, crate::error::Error>;

    async fn convert(&self);
}

#[async_trait::async_trait]
pub trait Repository<T>: Send + Sync + Clone + 'static {
    //this will call the provider. What are we thinking?
    type Provider: Provider; //why does this exist? Can you call provider from struct without initialising

    fn provider(&self) -> &Self::Provider;

    fn store_data(&self, order: Vec<T>); //internal event state just needs selection, event
}

#[derive(Debug, Clone)]
pub enum RequestType {
    //     UserHoldings(Option<&'a str>), //currency type
    UserHoldings(Vec<String>), //currency type
    UserInfo,
    UserCurrencyTradeHistory(String),
    UserOrderStats(Vec<String>),
}
// impl From<String> for RequestType {
//     fn from(value: String, params: Vec<String>) -> Self {
//         match value.as_str() {
//             "userHoldings" => Self::UserHoldings(vec![]),
//             "asfafasf" => Self::UserOrderStats(vec![]),
//             _ => Self::UserInfo,
//         }
//     }
// }
impl RequestType {
    fn from(value: String, params: Vec<String>) -> Self {
        match value.as_str() {
            "userHoldings" => Self::UserHoldings(vec![]),
            "asfafasf" => Self::UserOrderStats(vec![]),
            _ => Self::UserInfo,
        }
    }
    fn format_params(self) -> (String, String) {
        let mut str = String::new();
        match self {
            RequestType::UserHoldings(s) => {
                let mut string = String::new();
                //"category=linear&symbol=RNDRUSDT".to_string(),

                for ele in s {
                    println!("{:?}", ele);
                    string = format!("{ele}");
                }

                str.push_str(&format!("/v5/account/wallet-balance?{}", string));
                //account type- UNIFIED/CONTRACT/SPOT
                //or use /v5/asset/transfer/query-account-coins-balance"
                (str, string)
            }
            RequestType::UserInfo => {
                //https://bybit-exchange.github.io/docs/api-explorer/v5/position/position-info
                // _parameters = ["category", "inverse"];

                str.push_str("/v5/user/query-api"); //symbol
                (str, String::from(""))
            }
            RequestType::UserCurrencyTradeHistory(s) => {
                //category e.g. spot
                str.push_str(&format!("/v5/asset/delivery-record?{}", s));
                (str, s)
            }
            RequestType::UserOrderStats(s) => {
                let mut string = String::new();
                //"category=linear&symbol=RNDRUSDT".to_string(),

                for ele in s {
                    println!("{:?}", ele);
                    string = format!("{ele}");
                }
                println!("{:?}", string);
                str.push_str(&format!("/v5/order/history?{}", string)); //not sure if this is gna
                                                                        //work
                (str, string)
                //starttime-endtime modified
            }
        }
    }
}

#[tokio::test]
async fn test_thread() {
    let mut value = std::sync::Arc::new(std::sync::Mutex::new("arc original"));

    let mut value2 = std::sync::Arc::clone(&value);
    let mut value3 = std::sync::Arc::clone(&value);
    let thread = std::thread::spawn(move || {
        *value2.lock().unwrap() = "thread 2".into();
    });

    let thread123 = std::thread::spawn(move || {
        *value.lock().unwrap() = "thread 3".into();
    });

    // println!("{:?}", value.clone().lock().unwrap());
    // println!("{:?}", value2);
    // println!("{:?}", value3);

    // join!(thread, thread123);
}
