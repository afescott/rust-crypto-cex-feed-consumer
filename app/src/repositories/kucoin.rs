use axum::http::{HeaderMap, HeaderName, HeaderValue};
use base64::alphabet::STANDARD;
use hex::encode;
use hmac::{Hmac, Mac};
use ring::hmac::{sign, Key, HMAC_SHA256};
use serde_json::{json, map::Values, Value};
use sha2::Sha256;
use std::{collections::HashMap, fmt::Debug, str::FromStr, thread, time::Duration};

use kucoin_rs::{
    kucoin::{
        client::{Credentials, Kucoin, KucoinEnv},
        model::{
            market::{Currency, DailyStats, TradeHistories},
            user::Accounts,
        },
        trade::OrderInfoOptionals,
        utils::get_time,
    },
    reqwest::{self, Client},
};

use serde::de::DeserializeOwned;

use crate::{error::Error, repositories::response_to_json};

use super::{Provider, RequestType};

type HmacSha256 = Hmac<Sha256>;
pub enum KucoinReturnType {
    Account(Accounts),
    Currency(Currency),
    TradeHistory(TradeHistories),
    DailyStats(DailyStats),
}

#[derive(Clone)]
pub struct KucoinImplementation {
    pub client: Client,
}

#[async_trait::async_trait]
impl Provider for KucoinImplementation {
    fn new() -> Self {
        let client = reqwest::Client::builder()
            // .use_rustls_tls()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap();

        Self { client }
    }

    async fn get_user_info<T: DeserializeOwned + Debug, U: From<T>>(
        &self,
        request: RequestType,
    ) -> Result<Vec<U>, crate::error::Error> {
        let endpoint = request.format_url(crate::repositories::CexType::Kucoin);
        println!("{:?}", endpoint);
        let nonce = get_time().to_string();
        let d = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let api_key = dotenv::var("KUCOIN_KEY").unwrap();

        let secret_key = dotenv::var("KUCOIN_SECRET").unwrap();
        let passphrase = dotenv::var("KUCOIN_PASSPHRASE").unwrap();
        let str_to_sign = format!("{}{}{}", nonce, "GET", endpoint);

        println!("{:?}", str_to_sign);
        let key_sign = Key::new(HMAC_SHA256, &secret_key.as_bytes());

        let tag_secret = sign(&key_sign, str_to_sign.as_bytes());

        let sign_digest = encode(tag_secret.as_ref());

        println!("{:?}", sign_digest);

        // let passphrase_sign = Key::new(HMAC_SHA256, &secret_key.as_bytes());
        //
        // let tag_secret = sign(&passphrase_sign, passphrase.as_bytes());
        //
        // let passphrase_digest = encode(tag_secret.as_ref());

        // println!("{:?}", passphrase_digest);

        let mut headers = HeaderMap::new();

        headers.insert(
            HeaderName::from_static("kc-api-key"),
            HeaderValue::from_str(&api_key).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("kc-api-sign"),
            HeaderValue::from_str(&sign_digest).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("kc-api-timestamp"),
            HeaderValue::from_str(&nonce).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("kc-api-passphrase"),
            HeaderValue::from_str(&passphrase).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("kc-api-key-version"),
            HeaderValue::from_static("2"),
        );

        let client = reqwest::Client::new();

        let request = client
            .get(format!("https://api.kucoin.com{}", endpoint))
            .headers(headers);

        let response = response_to_json(
            request
                .send()
                .await
                .map_err(|e| Error::ReqwestError(e.to_string())),
        )
        .await
        .map_err(|e| Error::DeserializeError(e.to_string()))?;

        let mut vec = Vec::new();
        if let Some(value) = response["data"].as_array() {
            println!("{:?}", value);
            for ele in value {
                println!("{:?}", ele);
                let order: T = kucoin_rs::serde_json::from_value(ele.clone())
                    .map_err(|e| Error::DeserializeError(e.to_string()))?;

                println!("{:?}", order);

                vec.push(order.into());
            }
        } else {
        }

        Ok(vec)
    }
}
