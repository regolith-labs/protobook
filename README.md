# Protobook

**Protobook** is a Solana smart contract for issuing public, timebound orders to buy a given token at a fixed price. It allows anyone to securely execute a one-to-many OTC swap with participating public liquidity. In this way, it is a generic and permissionless orderbook protocol.

## Accounts
- [`Order`](api/src/state/order.rs) – An order is a public, timebound offer to buy a given token at a fixed price.
- [`Receipt`](api/src/state/receipt.rs) – A receipt tracks a deposit to fill an order.

## Instructions
- [`Cancel`](program/src/cancel.rs) – Cancels an order immediately.
- [`Close`](program/src/close.rs) – Closes an order account.
- [`Collect`](program/src/collect.rs) – Collects tokens from an expired order.
- [`Fill`](program/src/fill.rs) – Fills an order.
- [`Open`](program/src/open.rs) – Opens an order.
- [`Redeem`](program/src/redeem.rs) – Redeems a receipt for an expired order.

## How it works

A user can open an **order** by specifying the token they want to buy, the amount they want to buy, the token they want to sell, the amount they want to sell, and then locking the tokens they wish to sell in an escrow vault. Any user then can fill (or partially fill) the order by depositing the requested tokens into the escrow vault and receiving a **receipt** to track the deposit. 

When an order expires, its owner can collect either the tokens they wished to buy if the order was filled, or reclaim their original deposit if the order was not filled. During collection, an optional fee can be sent to a fee collector if the order was filled. Receipt holders can redeem their receipts to receive either the tokens offered if the order was filled, or their original deposit if the order was not filled. Once all tokens have been withdrawn from the escrow vaults, the order account can be closed and rent returned to its owner.

## Discussion

Protobook is built on the assumption that all order matching can happen offchain. It provides _only_ a system for issuing and managing public swap orders. These orders can represent one-off OTC trades between private parties or one-to-many swaps on a public exchange. An orderbook UI and trading bots can be readibly be built on the data structures provided by Protobook with full support for limit orders and immediate cancelling. In other words, Protobook puts the orders onchain and builds the "book" offchain. 

A book is simply an index of all open orders for a given market. To create a book, for example, one could filter for all Protobook orders between SOL and USDC mints. Then simply filter out the expired orders, index the orders into price ranges, and display the orders as a book to visualize the bid/ask spread and market depth. For orders batched into the same price range, an exchange can choose to fill orders with the best prices first. By assuming that all order matching happens offchain, Protobook greatly reduces contract complexity and transaction costs compared to existing orderbook protocols on Solana. 

## Get started

Compile your program:
```sh
steel build
```

Run unit and integration tests:
```sh
steel test
```
