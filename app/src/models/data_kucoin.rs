use serde_json::{json, Value};

pub fn kucoin() -> Value {
    json!({
    "items": [
      {
        "id": "64adfe86bab906ba75c683ca",
        "currency": "usdc",
        "balance": "$3,033.14",
        "available": "$1,299.69",
        "holds": 0
      },
      {
        "id": "64adfe86b86be6f5b23b66b2",
        "currency": "ethereum",
        "balance": "$2,211.37",
        "available": "$2,105.69",
        "holds": 0
      },
      {
        "id": "64adfe865252d7fcc55404fc",
        "currency": "ethereum",
        "balance": "$2,030.33",
        "available": "$2,475.67",
        "holds": 0
      },
      {
        "id": "64adfe866d1160028bc9d179",
        "currency": "usdc",
        "balance": "$3,171.76",
        "available": "$1,702.06",
        "holds": 0
      },
      {
        "id": "64adfe860f495adcdb053a91",
        "currency": "ethereum",
        "balance": "$3,396.66",
        "available": "$1,560.14",
        "holds": 0
      },
      {
        "id": "64adfe862d108d8be93c26f6",
        "currency": "ethereum",
        "balance": "$1,777.82",
        "available": "$2,579.37",
        "holds": 0
      }
    ]
    })
}
