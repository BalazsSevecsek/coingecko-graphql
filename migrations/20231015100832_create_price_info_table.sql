-- Add migration script here
CREATE TABLE price_info (
  crypto_id VARCHAR(255) NOT NULL,
  currency_ticker VARCHAR(255) NOT NULL,
  timestamp TIMESTAMPTZ NOT NULL,
  price NUMERIC(20, 8) NOT NULL,
  CONSTRAINT pk_price_info PRIMARY KEY (crypto_id, currency_ticker, timestamp)
);