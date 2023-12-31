#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(serde::Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Wallet {
    exhange: String,
    coin: String,
    account_type: String,
    balance: String,
    realised_pnl: Option<String>,
}

impl From<Wallet> for String {
    fn from(value: Wallet) -> Self {
        value.coin.to_string()
    }
}

#[derive(serde::Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct ByBitAccount {
    coin: Vec<ByBitWallet>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ByBitWallet {
    coin: String,
    #[serde(rename = "walletBalance")]
    wallet_balance: String,
    #[serde(rename = "availableToWithdraw")]
    available_to_withdraw: String,
    #[serde(rename = "cumRealisedPnl")]
    realised_pnl: String,
}
impl From<ByBitWallet> for Wallet {
    fn from(value: ByBitWallet) -> Self {
        Self {
            exhange: "Bybit".to_string(),
            coin: value.coin,
            account_type: "TODO".to_string(),
            balance: value.wallet_balance,
            realised_pnl: Some(value.realised_pnl),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct KucoinAccount {
    id: String,
    currency: String,
    #[serde(rename = "type")]
    account_type: String,
    balance: String,
    available: String,
    holds: u64,
}

impl From<KucoinAccount> for Wallet {
    fn from(value: KucoinAccount) -> Self {
        Self {
            exhange: "Kucoin".to_string(),
            coin: value.currency,
            account_type: value.account_type,
            balance: value.balance,
            realised_pnl: None,
        }
    }
}
