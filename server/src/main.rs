use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::cex_get::{bybit_thread_get_data, kucoin_thread_get_data};
use app::models::order::{BybitOrder, KucoinOrder, Order};
use app::models::wallet::{ByBitWallet, KucoinAccount, Wallet};
use app::repositories::mem::StorageRepo;
use axum::extract::Path;
use axum::Extension;
use axum::{routing::get, Router};
use axum_server::bind;
use axum_server::Handle;

mod cex_get;
mod config;

extern crate dotenv;

use dotenv::dotenv;
use tokio::join;

//Entry point for the application
#[tokio::main]
async fn main() {
    dotenv().ok();

    run_local_server().await;
}

async fn run_local_server() {
    let handle = Handle::new();
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("listening on 0.0.0.0:3000");
    //every time we receive a request. after request we need to store in state
    let storage_orders = Arc::new(StorageRepo::<Order>::new());

    let storage_wallet = Arc::new(StorageRepo::<Wallet>::new());

    join!(
        kucoin_thread_get_data::<KucoinOrder, Order>(
            "userOrderStats".to_string(),
            "category=spot".to_string(),
            Arc::clone(&storage_orders)
        ),
        kucoin_thread_get_data::<KucoinAccount, Wallet>(
            "userHoldings".to_string(),
            "accountType=spot".to_string(),
            Arc::clone(&storage_wallet)
        ),
        bybit_thread_get_data::<BybitOrder, Order>(
            "userOrderStats".to_string(),
            "category=spot".to_string(),
            Arc::clone(&storage_orders)
        ),
        bybit_thread_get_data::<ByBitWallet, Wallet>(
            "userHoldings".to_string(),
            "accountType=spot".to_string(),
            Arc::clone(&storage_wallet)
        ),
        bind(addr).handle(handle).serve(
            Router::new()
                .merge(route_api())
                .layer(Extension(Arc::clone(&storage_orders)))
                .layer(Extension(Arc::clone(&storage_wallet)))
                .into_make_service(),
        )
    );
}

fn route_api() -> Router {
    Router::new()
        .route("/bybit/:get_type", get(get_command::<Wallet>))
        .route("/kucoin/:get_type", get(get_command::<Order>))
}

async fn get_command<T>(
    Path(get_command): Path<String>,
    Extension(storage): Extension<Arc<StorageRepo<T>>>,
) -> String
where
    T: Clone + Debug + PartialEq,
    String: From<T>,
{
    println!("Hit");
    let state = storage.state.lock().unwrap();

    match get_command.as_str() {
        "list" => {
            format!("{:?}", state.values())
        }
        _ => {
            format!("{:?}", state.get(&get_command).unwrap())
        }
    }
}
