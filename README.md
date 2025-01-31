# Protobook

**Protobook** is a Solana program for issuing a public, timebound offer to buy a given token at a fixed price. It allows anyone to execute one-to-many OTC swaps with willing public liquidity. In this way, it represents a generalized and permissionless orderbook protocol.

## Accounts
- [`Order`](api/src/state/order.rs) – An order is a public, time-bound offer to exchange two tokens at a fixed price.
- [`Receipt`](api/src/state/receipt.rs) – A receipt is a deposit to fill an order.

## Instructions
- [`Cancel`](program/src/cancel.rs) – Cancels an order immediately.
- [`Claim`](program/src/claim.rs) – Claims escrowed tokens from an expired order.
- [`Close`](program/src/close.rs) – Closes an order account.
- [`Fill`](program/src/fill.rs) – Fills an order.
- [`Open`](program/src/open.rs) – Opens an order.
- [`Redeem`](program/src/redeem.rs) – Redeems a receipt for an expired order.

## Get started

Compile your program:
```sh
steel build
```

Run unit and integration tests:
```sh
steel test
```
