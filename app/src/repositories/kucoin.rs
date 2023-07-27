use std::{collections::HashMap, fmt::Debug};

use kucoin_rs::kucoin::{
    client::{Credentials, Kucoin, KucoinEnv},
    model::{
        market::{Currency, DailyStats, TradeHistories},
        user::Accounts,
    },
    trade::OrderInfoOptionals,
};
use serde::de::DeserializeOwned;

use super::{Provider, RequestType};

pub enum KucoinReturnType {
    Account(Accounts),
    Currency(Currency),
    TradeHistory(TradeHistories),
    DailyStats(DailyStats),
}

#[derive(Clone)]
pub struct KucoinImplementation {
    sign: Kucoin,
}

#[async_trait::async_trait]
impl Provider for KucoinImplementation {
    fn new() -> Self {
        let credentials = Credentials::new(
            dotenv::var("KUCOIN_PASSPHRASE").unwrap().as_str(),
            dotenv::var("KUCOIN_SECRET").unwrap().as_str(),
            dotenv::var("KUCOIN_KEY").unwrap().as_str(),
        );

        Self {
            sign: Kucoin::new(KucoinEnv::Live, Some(credentials)).expect("Invalid credentials"),
        }
    }

    async fn get_user_info<T>(
        &self,
        request_type: RequestType,
    ) -> Result<Vec<T>, crate::error::Error> {
        let mut daily_stats: Vec<T> = Vec::new();

        match request_type {
            RequestType::UserHoldings(s) => {
                let params = s
                    .split("&")
                    .collect::<Vec<_>>()
                    .chunks_exact(2)
                    .map(|chunk| (chunk[0].clone(), chunk[1].clone())) // map slices to tuples
                    .collect::<HashMap<&str, &str>>();

                let result = self
                    .sign
                    .get_accounts_list(params.get("currency").copied(), params.get("type").copied())
                    .await;
                todo!()
            }
            RequestType::UserInfo => {
                let result = self.sign.get_currencies().await;
                todo!()
            }
            RequestType::UserCurrencyTradeHistory(s) => {
                let params = s
                    .split("&")
                    .collect::<Vec<_>>()
                    .chunks_exact(2)
                    .map(|chunk| (chunk[0].clone(), chunk[1].clone())) // map slices to tuples
                    .collect::<HashMap<&str, &str>>();

                let _ = self
                    .sign
                    .get_trade_histories(params.get("symbol").ok_or(
                        crate::error::Error::ReqwestError("Missing currency parameter".to_string()),
                    )?)
                    .await;
                todo!()
            }
            RequestType::UserOrderStats(s) => {
                let result = self
                    .sign
                    .get_daily_stats(s.get(7..).ok_or(crate::error::Error::ReqwestError(
                        "Missing currency parameter".to_string(),
                    ))?)
                    .await
                    // .map_err(|err| crate::error::Error::ReqwestError("asfafaf".to_string()))
                    .map_err(|err| crate::error::Error::ReqwestError(err.to_string()));
                result.map(|x| daily_stats.push(x.into()));

                //TODO: Put values from each match statement into format then convert into T
                todo!()
            }
        }
    }
}
