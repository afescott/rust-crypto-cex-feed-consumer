use std::net::SocketAddr;
use std::sync::Arc;

use app::models::order::Order;
use app::models::wallet::Wallet;

use app::error::Error;
use app::repositories::bybit::ByBitImplementation;
use app::repositories::kucoin::KucoinImplementation;
use app::repositories::mem::{InnerRepository, StorageRepository};
use app::repositories::{Provider, Repository, RequestType};
use axum::extract::Path;
use axum::Extension;
use axum::{routing::get, Router};
use axum_server::bind;
use axum_server::Handle;
use config::Config;
use kucoin_rs::reqwest::Client;
use kucoin_rs::serde_json::{self, json, Value};
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
    let jim = Arc::new(ByBitImplementation {
        client: Client::new(),
    });

    let order_repo = StorageRepository::<_, Order>::new(Arc::clone(&jim));

    let wallet_repo = StorageRepository::<_, Wallet>::new(Arc::clone(&jim));

    let _result = bind(addr)
        .handle(handle)
        .serve(
            Router::new()
                .merge(route_api())
                .layer(Extension(order_repo))
                .layer(Extension(wallet_repo))
                .into_make_service(),
        )
        .await
        .unwrap();
}

fn route_api() -> Router {
    Router::new()
        // .route("/kucoin", get(|| async { "Kucoin  root" }))
        .route(
            "/bybit/:get_category/*params",
            get(get_bybit_command::<StorageRepository<ByBitImplementation, _>>),
        )
        .route(
            "/kucoin/:get_category/*params",
            get(get_kucoin_command::<StorageRepository<KucoinImplementation, _>>),
        )
    // .route(
    //     "/kucoin_wallet_balance/*params",
    //     get(get_wallet_balance::<StorageRepository<KucoinImplementation, Wallet>>),
    // )
}

// fn bybit() -> Router {
//     Router::new()
//         .route(
//             "/bybit",
//             get(get_coins_history::<StorageRepository<ByBitImplementation, Order>>),
//         )
//         .route("/wallet_balance", get(wallet_balance))
// }
async fn get_kucoin_command<T>(
    Path((get_category, params)): Path<(String, String)>,
    Extension(repository): Extension<T>,
) where
    T: Repository<Wallet>,
{
    //TODO: Start here
    // let result: RequestType = get_category.into();
    // let params = params.split("&").collect::<Vec<&str>>();
    // println!("{:?}", params);
    //
    // let result = repository
    //     .provider()
    //     .get_user_info::<Wallet>(RequestType::UserHoldings(
    //         params.iter().map(|&s| s.into()).collect(),
    //     ))
    //     .await
    //     .unwrap();
}

async fn get_bybit_command<T>(
    Path((get_command, params)): Path<(String, String)>,
    Extension(repository): Extension<T>,
) where
    T: Repository<Order>,
{
    let params = params.split("&").collect::<Vec<&str>>();
    println!("{:?}", params);
    // "category=linear&symbol=RNDRUSDT".to_string(),

    let result = repository
        .provider()
        .get_user_info::<Order>(RequestType::UserOrderStats(
            params.iter().map(|&s| s.into()).collect(),
        ))
        .await
        .unwrap();

    // for ele in result {
    //     repository.store_data(ele);
    // }

    todo!()
}
