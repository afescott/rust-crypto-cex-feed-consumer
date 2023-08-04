use serde_json::{json, Value};

pub fn kucoin_wallet() -> Value {
    json!({
    "data":
        [
              {
                "id": "5bd6e9216d99522a52e458d6",
                "currency": "BTC",
                "type": "trade",
                "balance": "1234356",
                "available": "1234356",
                "holds": "0"
              },
              {
                "id": "5bd6e9216d99522a52e458d6",
                "currency": "BTC",
                "type": "trade",
                "balance": "1234356",
                "available": "1234356",
                "holds": "0"
              },

        ]
        })
}

pub fn kucoin_order() -> Value {
    json!({
    "items": [
      {
        "id": "64adfe86bab906ba75c683ca",
        "currency": "usdc",
                 "type": "trade",

            "balance": "$3,033.14",
        "available": "$1,299.69",
        "holds": 0
      },
      {
        "id": "64adfe86b86be6f5b23b66b2",
        "currency": "ethereum",
                 "type": "spot",
        "balance": "$2,211.37",
        "available": "$2,105.69",
        "holds": 0
      },
      {
        "id": "64adfe865252d7fcc55404fc",
        "currency": "ethereum",

                 "type": "spot",
        "balance": "$2,030.33",
        "available": "$2,475.67",
        "holds": 0
      },
      {
        "id": "64adfe866d1160028bc9d179",
        "currency": "usdc",

                 "type": "margin",
        "balance": "$3,171.76",
        "available": "$1,702.06",
        "holds": 0
      },
    ]
    })
}
