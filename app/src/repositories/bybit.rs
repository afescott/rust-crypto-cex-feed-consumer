use std::{fmt::Debug, time::SystemTime};

use axum::http::{header::CONTENT_TYPE, HeaderMap, HeaderValue};
use hex::encode;
use kucoin_rs::reqwest::{self, Client};
use ring::hmac::{sign, Key, HMAC_SHA256};
use serde::de::DeserializeOwned;

use crate::{error::Error, repositories::response_to_json};

use super::{Provider, RequestType};

#[derive(Debug, Clone)]
pub struct ByBitImplementation {
    pub client: Client,
}

#[async_trait::async_trait]
impl Provider for ByBitImplementation {
    fn new() -> Self {
        let client = reqwest::Client::new();

        Self { client }
    }

    async fn get_user_info<T: DeserializeOwned + Debug, U: From<T>>(
        &self,
        request_type: RequestType,
    ) -> Result<Vec<U>, crate::error::Error> {
        let bybit_key = dotenv::var("BYBIT_KEY").unwrap();
        let bybit_secret = dotenv::var("BYBIT_SECRET").unwrap();
        let response = request_type.format_url(crate::repositories::CexType::Bybit);
        println!("{:?}", response);
        let params = request_type.clone().get_parameters();

        let d = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let key = Key::new(HMAC_SHA256, bybit_secret.as_bytes());

        let val = format!(
            "{time}{api_key}5000{params}",
            api_key = bybit_key,
            time = d,
            params = params //implement from for enum here?
        );

        let tag = sign(&key, &val.as_bytes());

        let result = encode(tag.as_ref());
        let mut headers = HeaderMap::new();

        headers.insert("X-BAPI-API-KEY", HeaderValue::from_str(&bybit_key).unwrap());

        headers.insert("X-BAPI-SIGN-TYPE", HeaderValue::from_static("2"));

        headers.insert("X-BAPI-RECV-WINDOW", HeaderValue::from_static("5000"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        headers.insert("X-BAPI-SIGN", HeaderValue::from_str(&result).unwrap());
        headers.insert("X-BAPI-SIGN-TYPE", HeaderValue::from_static("2"));
        headers.insert(
            "X-BAPI-TIMESTAMP",
            HeaderValue::from_str(&d.to_string()).unwrap(),
        );

        let request = self
            .client
            .get(format!("https://api.bybit.com{}", response))
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

        if let RequestType::UserHoldings(_) = request_type {
            let mut value = response["result"]["list"]
                .as_array()
                .unwrap()
                .iter()
                .map(|e| e["coin"].as_array().unwrap())
                .flatten()
                .collect::<Vec<_>>();

            for ele in value {
                let order: T = kucoin_rs::serde_json::from_value(ele.clone())
                    .map_err(|e| Error::DeserializeError(e.to_string()))?;

                vec.push(order.into());
            }
        } else {
            if let Some(value) = response["result"]["list"].as_array() {
                for ele in value {
                    let order: T = kucoin_rs::serde_json::from_value(ele.clone())
                        .map_err(|e| Error::DeserializeError(e.to_string()))?;

                    vec.push(order.into());
                }
            }
        }

        Ok(vec)
    }
}
