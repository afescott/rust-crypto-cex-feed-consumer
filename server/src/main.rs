use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;

use app::models::order::Order;
use app::models::wallet::Wallet;

use app::error::Error;
use app::repositories::bybit::ByBitImplementation;
use app::repositories::kucoin::KucoinImplementation;
use app::repositories::mem::StorageRepository;
use app::repositories::{Provider, Repository, RequestType};
use axum::extract::Path;
use axum::Extension;
use axum::{routing::get, Router};
use axum_server::bind;
use axum_server::Handle;
use config::Config;
use kucoin_rs::reqwest::Client;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use tokio::main;

mod config;
// mod tests;
//
extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    let result = dotenv().ok();
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    run_local_server().await;
}

async fn run_local_server() {
    let handle = Handle::new();
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    //initialise repository here as an arc state
    //every time we receive a request. after request we need to store in state
    //pass state to router subsequently
    let bybit_implementation = Arc::new(ByBitImplementation {
        client: Client::new(),
    });

    let kucoin_implementation = Arc::new(KucoinImplementation::new());

    //map out and rengineer this repository. doesn't make sense as is
    let order_repo_bybit =
        StorageRepository::<ByBitImplementation, Order>::new(Arc::clone(&bybit_implementation));
    let wallet_repo_bybit =
        StorageRepository::<ByBitImplementation, Wallet>::new(Arc::clone(&bybit_implementation));
    let wallet_repo_kucoin =
        StorageRepository::<KucoinImplementation, Wallet>::new(Arc::clone(&kucoin_implementation));
    let order_repo_kucoin =
        StorageRepository::<KucoinImplementation, Order>::new(Arc::clone(&kucoin_implementation));

    let _result = bind(addr)
        .handle(handle)
        .serve(
            Router::new()
                .merge(route_api())
                .layer(Extension(wallet_repo_bybit))
                .layer(Extension(order_repo_bybit))
                .layer(Extension(order_repo_kucoin))
                .layer(Extension(wallet_repo_kucoin))
                .into_make_service(),
        )
        .await
        .unwrap();
}

fn route_api() -> Router {
    Router::new()
        // .route("/kucoin", get(|| async { "Kucoin  root" }))
        .route(
            "/bybitorder/:order/*params",
            get(get_bybit_command::<StorageRepository<ByBitImplementation, Order>, Order>),
        )
        .route(
            "/bybitwallet/:wallet/*params",
            get(get_bybit_command::<StorageRepository<ByBitImplementation, Wallet>, Wallet>),
        )
        .route(
            "/kucoinwallet/:wallet/*params",
            get(get_kucoin_command::<StorageRepository<KucoinImplementation, Wallet>, Wallet>),
        )
        .route(
            "/kucoinorder/:order/*params",
            get(get_kucoin_command::<StorageRepository<KucoinImplementation, Order>, Order>),
        )
}

async fn get_kucoin_command<T, G>(
    Path((wallet, params)): Path<(String, String)>,
    Extension(repository): Extension<T>,
) where
    G: DeserializeOwned + Debug,
    T: Repository<G>,
{
    // let params = params.split("&").map(String::from).collect();
    // println!("{:?}, {:?}", wallet, params);
    let result = RequestType::from(wallet, params).unwrap();
    let result = repository
        .provider()
        .get_user_info::<G>(result)
        .await
        .unwrap();

    repository.store_data(result);
}

async fn get_bybit_command<T, G>(
    Path((get_command, params)): Path<(String, String)>,
    Extension(repository): Extension<T>,
) where
    G: DeserializeOwned + Debug,
    T: Repository<G>,
{
    let result = RequestType::from(get_command, params);

    let result = repository
        .provider()
        .get_user_info::<G>(result.unwrap())
        .await
        .unwrap();

    repository.store_data(result);

    todo!()
}
