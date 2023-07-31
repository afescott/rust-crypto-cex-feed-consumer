WIP:

Rust user api to get user order and wallet information via a generic axum web server. In-memory state mechanism to dynamically store unique data. Compared with similar data types from other exchanges to match into common structs. Result is an aggregator displaying a variety of user data.

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
