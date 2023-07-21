use std::{fmt::Debug, time::SystemTime};

use axum::http::{header::CONTENT_TYPE, HeaderMap, HeaderValue};
use hex::encode;
use kucoin_rs::{
    kucoin::client::{Credentials, Kucoin, KucoinEnv},
    reqwest::{self},
};
use ring::hmac::{sign, Key, HMAC_SHA256};
use serde::de::DeserializeOwned;

use crate::error::Error;

use super::{Provider, RequestType};

#[allow(dead_code)]
pub const KUCOIN_PASSPHRASE: &str = "GnsU_AU!-ah!np3";

#[allow(dead_code)]
pub const KUCOIN_SECRET: &str = "73085b2d-62ea-46a2-b643-de3277d18701";

#[allow(dead_code)]
pub const KUCOIN_KEY: &str = "6463d523360bf900012992f4";

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
            sign: Kucoin::new(KucoinEnv::Live, Some(credentials)).expect("safsaf"),
        }
    }

    async fn get_user_info<T: DeserializeOwned + Debug>(
        &self,
        request_type: RequestType,
    ) -> Result<Vec<T>, crate::error::Error> {
        let closure = move || println!("captured sfaf by value");

        closure();
        match request_type {
            RequestType::UserHoldings(s) => {
                let result = self.sign.get_accounts_list(None, None).await;
                println!("{:?}", s);
                todo!()
            }
            RequestType::UserInfo => {
                self.sign.get_currencies();
                todo!()
            }
            RequestType::UserCurrencyTradeHistory(s) => {
                self.sign.get_trade_histories("USDT");
                todo!()
            }
            RequestType::UserOrderStats(s) => {
                self.sign.get_daily_stats("USDT");
                todo!()
            }
        };

        // serde_json::from_value::<T>(
        //     response_to_json(
        //         request
        //             .send()
        //             .await
        //             .map_err(|e| Error::ReqwestError(e.to_string())),
        //     )
        //     .await
        //     .map_err(|e| crate::error::Error::ReqwestError(e.to_string()))?,
        // )
        // .map_err(|e| Error::DeserializeError(e.to_string()))
    }

    async fn convert(&self) {}
}
