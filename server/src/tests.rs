use app::repositories::{bybit::ByBitImplementation, kucoin::KucoinImplementation, Provider};
use kucoin_rs::kucoin::client::{Credentials, Kucoin, KucoinEnv};
use kucoin_rs::reqwest::Client;

use crate::run_local_server;

#[tokio::test]
// pub async fn kucoin_test() {
//     run_local_server().await;
//
//     let credentials = Credentials::new(
//         KUCOIN_KEY,        // API KEY
//         KUCOIN_SECRET,     // SECRET KEY
//         KUCOIN_PASSPHRASE, // PASSPHRASE
//     );
//
//     let body = reqwest::get("127.0.0.1:3001/kucoin_coin_history")
//         .await
//         .unwrap();
//     // let kucoin = KucoinImplement:qation::new();
//     //
//     // let api = Kucoin::new(KucoinEnv::Live, Some(credentials)).expect("safsaf");
//     //
//     // match api.get_orders(None).await {
//     //     Ok(d) => println!("{:#?}", d),
//     //     Err(e) => println!("Err: {:#?}", e),
//     // }
//     //
//     // match api.get_accounts_list(None, None).await {
//     //     Ok(d) => println!("{:#?}", d),
//     //     Err(e) => println!("Err: {:#?}", e),
//     // }
// }
#[tokio::test]
async fn test_bybit_order() {
    // let sign = ByBitImplementation::new();

    let jim = ByBitImplementation {
        client: Client::new(),
    };

    // get_coins_history::<OrderRepository<ByBitImplementation>>(Extension(repo)).await;
}
