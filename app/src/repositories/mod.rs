use serde::de::DeserializeOwned;

pub mod bybit;
pub mod kucoin;
pub mod mem;

#[async_trait::async_trait]
pub trait Provider: Send + Sync + 'static {
    fn new() -> Self;

    async fn get_user_info<T: DeserializeOwned + core::fmt::Debug>(
        &self,
        request: RequestType,
    ) -> Result<Vec<T>, crate::error::Error>;
}

#[async_trait::async_trait]
pub trait Repository<T>: Send + Sync + Clone + 'static {
    type Provider: Provider;

    fn provider(&self) -> &Self::Provider;

    fn store_data(&self, results: Vec<T>); //internal event state just needs selection, event
}

#[derive(Debug, Clone)]
pub enum RequestType {
    //     UserHoldings(Option<&'a str>), //currency type
    UserHoldings(String), //currency type
    UserInfo,
    UserCurrencyTradeHistory(String),
    UserOrderStats(String),
}

pub enum CexType {
    Bybit,
}

impl RequestType {
    pub fn from(value: String, params: String) -> core::result::Result<Self, String> {
        match value.as_str() {
            "userHoldings" => Ok(Self::UserHoldings(params)),
            "userOrderStats" => Ok(Self::UserOrderStats(params)),
            _ => Err(String::from("Please select a valid endpoint")), // Self::UserInfo,
        }
    }
    // pub fn from_new(value: String, params: Vec<String>) -> core::result::Result<Self, String> {
    //     match value.as_str() {
    //         "userHoldings" => Ok(Self::UserHoldings("asfsf".to_string())),
    //         "userOrderStats" => Ok(Self::UserOrderStats("asfaf".to_string())),
    //         _ => Err(String::from("Please select a valid endpoint")), // Self::UserInfo,
    //     }
    // }
    pub fn get_parameters(self) -> String {
        match self {
            RequestType::UserHoldings(p) => p,
            RequestType::UserInfo => todo!(),
            RequestType::UserCurrencyTradeHistory(p) => p,
            RequestType::UserOrderStats(p) => p,
        }
    }
    fn format_url(&self, cex: CexType) -> String {
        let mut str = String::new();
        match self {
            RequestType::UserHoldings(s) => {
                match cex {
                    CexType::Bybit => str.push_str(&format!("/v5/account/wallet-balance?{}", s)),
                }
                str.to_string()
            }
            RequestType::UserInfo => {
                //https://bybit-exchange.github.io/docs/api-explorer/v5/position/position-info
                // _parameters = ["category", "inverse"];
                match cex {
                    CexType::Bybit => str.push_str("/v5/user/query-api"),
                }
                str.to_string()
            }
            RequestType::UserCurrencyTradeHistory(s) => {
                //category e.g. spot
                match cex {
                    CexType::Bybit => str.push_str(&format!("/v5/asset/delivery-record?{}", s)),
                }
                str.to_string()
            }
            RequestType::UserOrderStats(s) => {
                match cex {
                    CexType::Bybit => str.push_str(&format!("/v5/order/history?{}", s)),
                }

                println!("{:?}", str);

                str.to_string()
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
