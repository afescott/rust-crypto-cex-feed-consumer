use std::{sync::Arc, thread, time::Duration};

use app::repositories::{
    bybit::ByBitImplementation, kucoin::KucoinImplementation, mem::StorageRepo, Provider,
    RequestType, StorageRepository,
};
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub async fn kucoin_thread_get_data<T, U>(get_type: String, params: String)
where
    U: From<T> + Clone + Debug + Send,
    T: DeserializeOwned + Debug,
    (std::string::String, std::string::String): From<U>, // U: From<(String, String)>,
{
    let result = tokio::spawn(async move {
        let bybit_implementation = KucoinImplementation {
            client: Client::new(),
        };

        //every time we receive a request. after request we need to store in state
        let storage_orders = Arc::new(StorageRepo::<U>::new());

        let result = RequestType::from(get_type, params);
        println!("Thread");
        loop {
            let result = bybit_implementation
                .get_user_info::<T, U>(result.clone().unwrap())
                .await
                .unwrap();

            storage_orders.store_data(result);

            thread::sleep(Duration::from_secs(3));
        }
    });

    result.await.unwrap();
}

pub async fn bybit_thread_get_data<T, U>(get_type: String, params: String)
where
    U: From<T> + Clone + Debug + Send,
    T: DeserializeOwned + Debug,
    (std::string::String, std::string::String): From<U>, // U: From<(String, String)>,
{
    let result = tokio::spawn(async move {
        let bybit_implementation = ByBitImplementation {
            client: Client::new(),
        };

        //every time we receive a request. after request we need to store in state
        let storage_orders = Arc::new(StorageRepo::<U>::new());

        let result = RequestType::from(get_type, params);
        println!("Thread");
        loop {
            let result = bybit_implementation
                .get_user_info::<T, U>(result.clone().unwrap())
                .await
                .unwrap();

            storage_orders.store_data(result);

            thread::sleep(Duration::from_secs(3));
        }
    });

    result.await.unwrap();
}
