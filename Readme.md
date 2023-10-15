# coingeck-graphql

## Db

I extended the original concept with the normal non-crypto currency (currency_ticker) against which I retrieve prices.
I used a small in memory cache to check the input against that.

Also I had to expose an endpoint to get the coingecko ids for a cryptocurrency ticker otherwise it would be ambigous to the user which crypto is being queried.

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

Result:

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

Result:

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
