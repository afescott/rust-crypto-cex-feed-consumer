use axum::http::{HeaderMap, HeaderName, HeaderValue};
use hex::encode;
use hmac::{Hmac, Mac};
use ring::hmac::{sign, Key, HMAC_SHA256};
use sha2::Sha256;
use std::{collections::HashMap, fmt::Debug, thread, time::Duration};

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

use crate::error::Error;

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

    async fn get_user_info<T: DeserializeOwned + core::fmt::Debug>(
        &self,
        request: RequestType,
    ) -> Result<Vec<T>, crate::error::Error> {
        let url = "https://api.kucoin.com/api/v2/sub/user";
        let req_url = reqwest::Url::parse(&url).unwrap();
        let endpoint = request.format_url(crate::repositories::CexType::Kucoin);
        let nonce = get_time().to_string();

        let api_key = dotenv::var("KUCOIN_KEY").unwrap();

        let secret_key = dotenv::var("KUCOIN_SECRET").unwrap();
        let passphrase = dotenv::var("KUCOIN_PASSPHRASE").unwrap();

        let str_to_sign = format!("{}{}{}", nonce, "GET", "/api/v2/sub/user");
        //
        // println!("{:?}", str_to_sign);
        // let key_sign = Key::new(HMAC_SHA256, &secret_key.as_bytes());
        //
        // let tag_secret = sign(&key_sign, str_to_sign.as_bytes());
        //
        // let sign_digest = encode(tag_secret.as_ref());
        //
        // let key = Key::new(HMAC_SHA256, &secret_key.as_bytes());
        //
        // let tag_passphrase = sign(&key, passphrase.as_bytes());
        //
        // let passphrase_digest = encode(tag_passphrase.as_ref());

        let mut hmac_sign = <HmacSha256 as hmac::Mac>::new_varkey(secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        hmac_sign.input(str_to_sign.as_bytes());
        let sign_result = hmac_sign.result();
        let sign_bytes = sign_result.code();
        let sign_digest = encode(&sign_bytes.as_ref());
        let mut hmac_passphrase =
            HmacSha256::new_varkey(secret_key.as_bytes()).expect("HMAC can take key of any size");
        hmac_passphrase.input(passphrase.as_bytes());
        let passphrase_result = hmac_passphrase.result();
        let passphrase_bytes = passphrase_result.code();
        let passphrase_digest = encode(&passphrase_bytes.as_ref());
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
            HeaderValue::from_str(&passphrase_digest).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("kc-api-key-version"),
            HeaderValue::from_static("2"),
        );

        let client = reqwest::Client::new();
        //
        let request = client
            .get(req_url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| Error::ReqwestError(e.to_string()));

        Ok(Vec::new())
    }
}
