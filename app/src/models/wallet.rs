#![allow(dead_code)]

use serde::Deserialize;
use serde_json::Value;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Wallet {
    exhange: String,
    coin: String,
    account_type: String,
    balance: String,
    realised_pnl: Option<String>,
}

impl From<Wallet> for (String, String) {
    fn from(value: Wallet) -> Self {
        (value.coin.to_string(), value.exhange.to_string())
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

// use serde not kucoin::serde
// impl<'de> serde::Deserialize<'de> for ByBitWallet {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let s: &str = serde::Deserialize::deseriali(deserializer)?;
//
//         // println!("{:?}", D);
//         todo!()
//         // do better hex decoding than this
//         // u64::from_str_radix(&s[2..], 16)
//         //     .map(Account)
//         //     .map_err(D::Error::custom)
//     }
// }
//
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
