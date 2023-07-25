#![allow(dead_code)]

use serde::Deserialize;

use crate::repositories::{Provider, Repository};

// accounttype: compulsory parameter, coin optional
#[derive(serde::Deserialize, Debug, Clone)]
pub struct Wallet {
    coin: String,
    account_type: String,
    balance: f64,
    available_balance: f64,
}

impl<T: Provider + Clone> Repository<Wallet>
    for crate::repositories::mem::StorageRepository<T, Wallet>
{
    type Provider = T;

    fn provider(&self) -> &Self::Provider {
        &self.provider
    }

    fn store_data(&self, results: Vec<Wallet>) {
        let result = &self.state.lock().unwrap();
    }
}

#[derive(Deserialize, Debug)]
struct Obj<T> {
    items: Vec<T>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ByBitAccount {
    coin: String,
    transferbalance: String,
    walletbalance: String,
    #[serde(rename = "type")]
    trade_type: String,
    holds: u64,
}
impl From<ByBitAccount> for Wallet {
    fn from(mut value: ByBitAccount) -> Self {
        value.transferbalance.remove(0);
        value.walletbalance.remove(0);

        value.transferbalance.remove(1);
        value.walletbalance.remove(1);

        println!("{:?}", value.walletbalance);
        Self {
            coin: value.coin,
            account_type: "none".to_string(),

            balance: value.transferbalance.parse::<f64>().unwrap(),
            available_balance: value.walletbalance.parse::<f64>().unwrap(),
        }
    }
}
#[derive(Deserialize, Debug, Clone)]
pub struct KucoinAccount {
    id: String,
    currency: String,
    //#[serde(rename = "type")]
    //trade_type: String,
    balance: String,
    available: String,
    holds: u64,
}
impl From<KucoinAccount> for Wallet {
    fn from(mut value: KucoinAccount) -> Self {
        value.available.remove(0);
        value.balance.remove(0);

        value.available.remove(1);
        value.balance.remove(1);

        println!("{:?}", value.balance);
        Self {
            account_type: "None".to_string(),
            available_balance: value.balance.parse::<f64>().unwrap(),
            coin: value.currency,
            balance: value.available.parse::<f64>().unwrap(),
        }
    }
}

#[tokio::test]
async fn test_deserialize() {
    let bybit = crate::models::data_bybit::bybit();
    let issues = serde_json::from_value::<Obj<ByBitAccount>>(bybit).unwrap();
    for i in issues.items {
        let safaf: Wallet = i.clone().into();

        println!("{:#?}", i);
    }

    let kucoin = crate::models::data_kucoin::kucoin();
    let issues = serde_json::from_value::<Obj<KucoinAccount>>(kucoin).unwrap();
    for i in issues.items {
        let safaf: Wallet = i.clone().into();

        println!("{:#?}", i);
    }
}
