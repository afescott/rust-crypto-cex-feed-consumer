Bybit and Kucoin Aggregator to retrieve wallet and spot order information. Comparisons are made to find matching tokens between exchanges (token pairs). Axum routing is used to provide api endpoints for a client to retrieve wallet/token related info. Rust memory state mechanism to dynamically store user info

Running:

Create .env file in root with following 

KUCOIN_PASSPHRASE =
KUCOIN_SECRET =
KUCOIN_KEY =
BYBIT_KEY  =
BYBIT_SECRET =

Cargo run server. Run endpoints. Examples:

http://localhost:3000/kucoinwallet/userHoldings/accountType=spot

http://localhost:3000/kucoinorder/userOrderStats/symbol=BTC-USDT

http://localhost:3000/bybitorder/userOrderStats/category=spot

http://localhost:3000/bybitwallet/userHoldings/accountType=spot


TODO: Add post/put/delete mechanism and additional exchanges. Decentralised wallets
