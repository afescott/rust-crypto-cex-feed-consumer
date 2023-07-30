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
    UserCurrencyTradeHistory(String),
    UserOrderStats(String),
}

pub enum CexType {
    Bybit,
    Kucoin,
}

impl RequestType {
    pub fn from(value: String, params: String) -> core::result::Result<Self, String> {
        match value.as_str() {
            "userHoldings" => Ok(Self::UserHoldings(params)),
            "userOrderStats" => Ok(Self::UserOrderStats(params)),
            _ => Err(String::from("Please select a valid endpoint")), // Self::UserInfo,
        }
    }
    pub fn get_parameters(self) -> String {
        match self {
            RequestType::UserHoldings(p) => p,
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
                    CexType::Kucoin => str.push_str(&format!("/api/v1/accounts?{}", s)),
                }
                str.to_string()
            }
            RequestType::UserCurrencyTradeHistory(s) => {
                //category e.g. spot
                match cex {
                    CexType::Bybit => str.push_str(&format!("/v5/asset/delivery-record?{}", s)),
                    CexType::Kucoin => str.push_str(&format!("/api/v1/market/histories?{}", s)),
                }
                str.to_string()
            }
            RequestType::UserOrderStats(s) => {
                match cex {
                    CexType::Bybit => str.push_str(&format!("/v5/order/history?{}", s)),
                    CexType::Kucoin => str.push_str(&format!("/api/v1/market/stats?{}", s)),
                }

                println!("{:?}", str);

                str.to_string()
                //starttime-endtime modified
            }
        }
    }
}
