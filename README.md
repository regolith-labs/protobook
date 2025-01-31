# Protobook

**Protobook** is a smart contract for issuing public, timebound offers to swap two tokens at a fixed price. It allows a private party to execute a one-to-many OTC exchange with public liquidity. In this way, it represents a generalized orderbook protocol.

## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions
- [`Cancel`](program/src/cancel.rs) – Cancels an order immediately.
- [`Claim`](program/src/claim.rs) – Claims escrowed tokens from an expired order.
- [`Close`](program/src/close.rs) – Closes an order account.
- [`Fill`](program/src/fill.rs) – Fills an order.
- [`Open`](program/src/open.rs) – Opens an order.
- [`Redeem`](program/src/redeem.rs) – Redeems a receipt for an expired order.

## State
- [`Order`](api/src/state/order.rs) – An order is a public, time-bound offer to exchange two tokens at a fixed price.
- [`Receipt`](api/src/state/receipt.rs) – A receipt is a public deposit to fill an order.

## Get started

Compile your program:
```sh
steel build
```

Run unit and integration tests:
```sh
steel test
```
