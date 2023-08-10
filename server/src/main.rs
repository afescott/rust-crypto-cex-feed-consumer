use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::cex_get::{bybit_thread_get_data, kucoin_thread_get_data};
use app::models::order::{BybitOrder, KucoinOrder, Order};
use app::models::wallet::{ByBitWallet, KucoinAccount, Wallet};
use app::repositories::bybit::ByBitImplementation;
use app::repositories::kucoin::KucoinImplementation;
use app::repositories::mem::StorageRepo;
use app::repositories::{Provider, RequestType, StorageRepository};
use axum::extract::Path;
use axum::Extension;
use axum::{routing::get, Router};
use axum_server::bind;
use axum_server::Handle;
use kucoin_rs::reqwest::Client;
use serde::de::DeserializeOwned;

mod cex_get;
mod config;

extern crate dotenv;

use dotenv::dotenv;

//Entry point for the application
#[tokio::main]
async fn main() {
    dotenv().ok();

    tokio::select! {
    _ = kucoin_thread_get_data::<KucoinOrder, Order>("userOrderStats".to_string(), "category=spot".to_string()) => println!("Kucoin thread crash Order"),
    _ = kucoin_thread_get_data::<KucoinAccount, Wallet>("userHoldings".to_string(), "accountType=spot".to_string()) =>
        println!("Kucoin thread crash wallet"),
    _ = bybit_thread_get_data::<BybitOrder, Order>("userOrderStats".to_string(), "category=spot".to_string()) =>
        println!("Bybit thread crash Order"),
    _ = bybit_thread_get_data::<ByBitWallet, Wallet>("userHoldings".to_string(), "accountType=spot".to_string()) =>
        println!("Bybit thread crash wallet"),
    _ =  run_local_server() => println!("Router crashed"),
    }

    run_local_server().await;
}

async fn run_local_server() {
    let handle = Handle::new();
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));

    //initialise providers here
    let bybit_implementation = ByBitImplementation {
        client: Client::new(),
    };

    let kucoin_implementation = KucoinImplementation {
        client: Client::new(),
    };

    //every time we receive a request. after request we need to store in state
    let storage_orders = Arc::new(StorageRepo::<Order>::new());

    let storage_wallet = Arc::new(StorageRepo::<Wallet>::new());

    let _result = bind(addr)
        .handle(handle)
        .serve(
            Router::new()
                .merge(route_api())
                .layer(Extension(kucoin_implementation))
                .layer(Extension(bybit_implementation))
                .layer(Extension(storage_orders))
                .layer(Extension(storage_wallet))
                .into_make_service(),
        )
        .await
        .unwrap();
}

fn route_api() -> Router {
    Router::new()
        .route(
            "/bybitorder/:order/*params",
            get(get_bybit_command::<BybitOrder, Order>),
        )
        .route(
            "/bybitwallet/:wallet/*params",
            get(get_bybit_command::<ByBitWallet, Wallet>),
        )
        .route(
            "/kucoinwallet/:wallet/*params",
            get(get_kucoin_command::<KucoinAccount, Wallet>),
        )
        .route(
            "/kucoinorder/:order/*params",
            get(get_kucoin_command::<KucoinOrder, Order>),
        )
}

async fn get_kucoin_command<T, U>(
    Path((get_command, params)): Path<(String, String)>,
    Extension(repository): Extension<KucoinImplementation>,
    Extension(storage): Extension<Arc<StorageRepo<U>>>,
) where
    T: DeserializeOwned + Debug,
    U: From<T> + Clone,
    (std::string::String, std::string::String): From<U>, // U: From<(String, String)>,
{
    let result = RequestType::from(get_command, params);

    let result = repository
        .get_user_info::<T, U>(result.unwrap())
        .await
        .unwrap();

    storage.store_data(result);

    todo!()
}

async fn get_bybit_command<T, U>(
    Path((get_command, params)): Path<(String, String)>,
    Extension(repository): Extension<ByBitImplementation>,
    Extension(storage): Extension<Arc<StorageRepo<U>>>,
) where
    U: From<T> + Clone,
    T: DeserializeOwned + Debug,
    (std::string::String, std::string::String): From<U>, // U: From<(String, String)>,
{
    let result = RequestType::from(get_command, params);

    let result = repository
        .get_user_info::<T, U>(result.unwrap())
        .await
        .unwrap();

    storage.store_data(result);

    todo!()
}
