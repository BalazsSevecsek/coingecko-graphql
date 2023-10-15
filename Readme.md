# coingeck-graphql

## Setup

Prerequisites:

- docker is installed along with docker compose
- cargo is installed through [rustup](https://rustup.rs/)

In the root folder of the project execute the following:

- cargo build
- docker compose up
- cargo run
- open the browser with the [following link](http://localhost:8000/)

## Important notes

- I extended the original concept with the normal currency (currency_ticker) against which I retrieve prices. There is an additional endpoint by which the user can figure out what sort of "eth" ticker types exist. The different variants are called crypto_ids and they form the basis of any subsequent queries.
- I used a small in memory cache for storing crypto_ids that gets filled up at the beginning of the server lifecycle.
- Subscription mostly goes from the db if the record is not older than 10 minutes (coingecko states that they refresh current prices every 5 minutes, however that was not what I experienced)
- Migration running should not be a part of the application initialization cycle, however it is convenient for this demo (should be part of CI).
- I do not try to handle the data granularity of the historical endpoint, however it would be better to sort and create separate tables according to daily/hourly/5min prices as stated in their documentation:
  - ```
    1 day from current time = 5 minute interval data
    2 - 90 days of date range = hourly data
    above 90 days of date range = daily data (00:00 UTC)
    ```
- The historical data is stored in the db however it is not queried from there for now.

## How to run migrations:

Run the following commands:

- `install cargo install sqlx-cli`
- `sqlx migrate run --database-url "postgres://user:pass@localhost/prices"`

## Example queries

### To get the cryptoIds pertaining to a crypto ticker such as "eth"

```

{
cryptoTickerIds(cryptoTicker: "eth")
}

```

Query Result:

```

{
"data": {
"cryptoTickerIds": [
"bridged-wrapped-ether-starkgate",
"ethereum",
"ethereum-wormhole"
]
}
}

```

### To get the current price by cryptoId and the currency ticker:

```

{
getCurrentPrice(cryptoId: "bitcoin", currencyTicker: "usd") {
cryptoId
currencyTicker
price
}
}

```

Query Result:

```

{
"data": {
"getCurrentPrice": {
"cryptoId": "bitcoin",
"currencyTicker": "usd",
"price": 26903
}
}
}

```

### Get the historical prices

```

{
getHistoricalPrice(
cryptoId: "bitcoin"
currencyTicker: "usd"
from: "2023-09-01T00:00:00.12345Z"
to: "2022-10-15T00:00:00.12345Z"
) {
cryptoId
currencyTicker
price
}
}

```

Query Result:

```

{
"data": {
"getHistoricalPrice": [
{
"cryptoId": "zombie-inu-2",
"currencyTicker": "usd",
"price": 0.001330072162941906,
"utcTimestamp": "2023-08-12T01:01:26Z"
},
{
"cryptoId": "zombie-inu-2",
"currencyTicker": "usd",
"price": 0.001330303633929849,
"utcTimestamp": "2023-08-12T02:01:00Z"
},
{
"cryptoId": "zombie-inu-2",
"currencyTicker": "usd",
"price": 0.0013305024345516816,
"utcTimestamp": "2023-08-12T03:01:41Z"
},
....
]
}
}

```

### Subscribe to latest price changes:

```

subscription{
currentPrice(cryptoId:"bitcoin", currencyTicker:"usd"){cryptoId, currencyTicker, price}
}

```

Query Result:

```

{
"data": {
"currentPrice": {
"cryptoId": "bitcoin",
"currencyTicker": "usd",
"price": 27068
}
}
}

```

```

```
