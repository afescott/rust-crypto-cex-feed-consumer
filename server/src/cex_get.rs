use std::{sync::Arc, thread, time::Duration};

use app::repositories::{
    bybit::ByBitImplementation, kucoin::KucoinImplementation, mem::StorageRepo, Provider,
    RequestType, StorageRepository,
};
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub async fn kucoin_thread_get_data<T, U>(
    get_type: String,
    params: String,
    storage: Arc<StorageRepo<U>>,
) where
    U: From<T> + Clone + Debug + Send + PartialEq + 'static,
    T: DeserializeOwned + Debug,
    String: From<U>, // U: From<(String, String)>,
{
    let result = tokio::spawn(async move {
        let bybit_implementation = KucoinImplementation {
            client: Client::new(),
        };

        //every time we receive a request. after request we need to store in state
        let result = RequestType::from(get_type, params);

        loop {
            let result = bybit_implementation
                .get_user_info::<T, U>(result.clone().unwrap())
                .await
                .unwrap();
            println!("{:?}", result);

            storage.store_data(result);

            thread::sleep(Duration::from_secs(3));
        }
    });

    result.await.unwrap();
}

pub async fn bybit_thread_get_data<T, U>(
    get_type: String,
    params: String,
    bybit_storage: Arc<StorageRepo<U>>,
) where
    U: From<T> + Clone + Debug + Send + PartialEq + 'static,
    T: DeserializeOwned + Debug,
    String: From<U>, // U: From<(String, String)>,
{
    let result = tokio::spawn(async move {
        let bybit_implementation = ByBitImplementation {
            client: Client::new(),
        };

        //every time we receive a request. after request we need to store in state
        let result = RequestType::from(get_type, params);
        loop {
            let result = bybit_implementation
                .get_user_info::<T, U>(result.clone().unwrap())
                .await
                .unwrap();

            bybit_storage.store_data(result);
            thread::sleep(Duration::from_secs(3));
        }
    });

    result.await.unwrap();
}
