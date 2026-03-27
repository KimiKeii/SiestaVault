# SiestaVault  
A micro-NFT marketplace for Southeast Asian fan artists to mint, sell, and earn royalties on their digital art using Stellar.

---

## Problem and Solution

### Problem  
Fan artists in Southeast Asia who create game-inspired digital art have no accessible platform to monetize their work as NFTs. Existing platforms charge high fees and are primarily designed for Western audiences, making them less accessible and less relevant to SEA creators.

### Solution  
SiestaVault is a micro-NFT marketplace built on Stellar, designed specifically for SEA fan artists. It offers low minting fees, supports local payment context, and provides a discovery feed tailored to gaming fandoms. Artists can mint NFTs, set royalties, and earn from secondary sales.

---

## Timeline

- Week 1: Smart contract development (NFT minting, ownership, duplicate prevention)  
- Week 2: Frontend marketplace UI (basic listing, minting interface)  
- Week 3: Wallet integration and testnet interaction  
- Week 4: Discovery feed and royalty distribution logic  
- Final Week: Testing, debugging, and testnet deployment  

---

## Stellar Features Used

- Soroban Smart Contracts  
  - NFT minting logic  
  - Ownership storage  
  - Royalty and reward logic  
  - Duplicate prevention using on-chain storage  

- Stellar DEX (planned for future)  
  - Secondary marketplace trading  
  - Liquidity for NFT resale  

---

## Prerequisites

- Rust toolchain (latest stable)  
- Soroban CLI (latest version)  

---
Contract ID:CACCXDZXKS24GVOULDAKFIMW6YVA247IFRUDLLD3XTKT55JQDIBVQZRF

Link: https://stellar.expert/explorer/testnet/contract/CACCXDZXKS24GVOULDAKFIMW6YVA247IFRUDLLD3XTKT55JQDIBVQZRF
---
## Build, Test, Deploy, and Sample Invocation

```bash
# Build the contract
soroban contract build

# Run tests
cargo test

# Deploy to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellaroid.wasm \
  --network testnet

# Sample CLI invocation (MVP function)
soroban contract invoke \
  --id <CONTRACT_ID> \
  -- \
  register_certificate \
  --user GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX \
  --cert_hash 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20

  ```
# MIT License
Copyright (c) 2026

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
