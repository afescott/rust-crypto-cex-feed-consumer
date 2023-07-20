#![warn(dead_code)]
use serde_json::{json, Value};

pub fn bybit() -> Value {
    json!(  {
       "items": [
         {
           "coin": "usdc",
           "transferbalance": "$1,011.36",
           "walletbalance": "$1,319.40",
           "type": "normal",
           "holds": 0
         },
         {
           "coin": "usdc",
           "transferbalance": "$1,363.37",
           "walletbalance": "$3,525.97",
           "type": "normal",
           "holds": 0
         },
         {
           "coin": "usdc",
           "transferbalance": "$2,189.24",
           "walletbalance": "$2,005.60",
           "type": "normal",
           "holds": 0
         },
         {
           "coin": "usdt",
           "transferbalance": "$3,558.25",
           "walletbalance": "$2,414.91",
           "type": "normal",
           "holds": 0
         },
         {
           "coin": "usdc",
           "transferbalance": "$3,985.07",
           "walletbalance": "$1,937.96",
           "type": "normal",
           "holds": 0
         }
       ]
    }
        )
}
