# Protobook

**Protobook** is a Solana smart contract for issuing public, timebound orders to buy a given token at a fixed price. It allows anyone to execute secure, one-to-many OTC swaps with participating public liquidity. In this way, it represents a generic permissionless orderbook protocol.

## Accounts
- [`Order`](api/src/state/order.rs) – An order is a public, time-bound offer to exchange two tokens at a fixed price.
- [`Receipt`](api/src/state/receipt.rs) – A receipt tracks a deposit to fill an order.

## Instructions
- [`Cancel`](program/src/cancel.rs) – Cancels an order immediately.
- [`Close`](program/src/close.rs) – Closes an order account.
- [`Collect`](program/src/collect.rs) – Collects tokens from an expired order.
- [`Fill`](program/src/fill.rs) – Fills an order.
- [`Open`](program/src/open.rs) – Opens an order.
- [`Redeem`](program/src/redeem.rs) – Redeems a receipt for an expired order.

## How it works

A user opens an order by specifying the token they want to buy, the amount they want to buy, the token they want to sell, the amount they want to sell, and then locking the tokens they wish to sell in an escrow vault. Any user can fill (or partially fill) the order by depositing the requested tokens into the escrow vault and receiving a receipt to track the deposit. 

When an order expires, its owner can collect it to receive either the tokens they wished to buy if the order was filled, or reclaim their original deposit if the order was not filled. An optional fee can be sent to a fee collector if the order was filled. Additionally, receipt holders can redeem their receipts for expired orders to receive either the tokens offered if the order was filled, or their original deposit if the order was not filled.

Once all tokens have been transferred from the escrow vaults, the order account can be closed and rent returned to its owner.

## Get started

Compile your program:
```sh
steel build
```

Run unit and integration tests:
```sh
steel test
```
