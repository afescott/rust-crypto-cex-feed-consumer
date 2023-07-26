use std::{collections::HashMap, fmt::Debug};

use kucoin_rs::kucoin::{
    client::{Credentials, Kucoin, KucoinEnv},
    trade::OrderInfoOptionals,
};
use serde::de::DeserializeOwned;

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
        // request_type.format_url(super::CexType::Kucoin);

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
                self.sign.get_currencies();
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
                    .get_trade_histories(
                        params
                            .get("symbol")
                            .ok_or(crate::error::Error::ReqwestError(
                                "Missing currency parameter".to_string(),
                            ))
                            .unwrap(),
                    )
                    .await;
                todo!()
            }
            RequestType::UserOrderStats(s) => {
                // self.sign.get_orders(Some(OrderInfoOptionals {
                //     status: Some("afsafs"),
                //     current_page: Some(22),
                //     end_at: Some(25),
                //     page_size: Some(25),
                //     start_at: Some(12),
                //     side: Some("asfaf"),
                //     symbol: Some("asfasf"),
                //     trade_type: Some("asfaf"),
                //     r#type: Some("asfas"), // type : Some("asfaf")
                // }));
                self.sign.get_daily_stats("USDT").await;
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
}
