# OrbitPay Contracts

Soroban-ready contract scaffold for OrbitPay Kit.

## Contract Goal

The contract package defines a funding intent layer for C-address onboarding:

- app creates a funding intent,
- user funds the intent with USDC/XLM,
- settlement can be claimed by the destination C-address,
- expired intents can be cancelled,
- server/webhook layer can watch contract events.

## Current State

This repo is intentionally an MVP scaffold. It includes the intended storage model and contract entry points, but production contributors need to complete:

- Soroban SDK compilation setup,
- token transfer integration,
- auth boundaries,
- event schema,
- full unit tests,
- audit review.

## Suggested Commands

After installing Rust and Soroban tooling:

```powershell
cargo test
soroban contract build
```

## Contributor Issue Seeds

- Implement token transfer calls using Soroban token client.
- Add admin-free merchant intent creation.
- Emit canonical `funding_intent_created` and `funding_intent_settled` events.
- Add expiry cancellation test cases.
- Add integration docs for `orbitpay-server`.

