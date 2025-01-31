# Protobook

**Protobook** is a Solana smart contract for issuing public, timebound orders to buy a given token at a fixed price. It allows anyone to execute secure, one-to-many OTC swaps with participating public liquidity. In this way, it represents a generic permissionless orderbook protocol.

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

## How it works

A user opens an order by specifying the token they want to buy, the amount they want to buy, the token they want to sell, the amount they want to sell, and locking the tokens they wish to sell in an order escrow vault. Any user can fill (or partially fill) an order by depositing the desired tokens into the order's escrow vault, and receiving a receipt to mark the deposit. 

When the order expires, the owner of the order can either claim the tokens they wished to buy if the order was filled, or take back their originally deposited tokens they wished to sell if the order was not filled. If the order is filled, an optional fee can be collected by a fee collector in the form of a % of the received tokens. Additionally, receipt holders can redeem their receipt for either the tokens offered if the order was filled, or the tokens they deposited if the order was not filled.

If all tokens have been claimed from the escrow vaults, the order account can be closed.

## Get started

Compile your program:
```sh
steel build
```

Run unit and integration tests:
```sh
steel test
```
