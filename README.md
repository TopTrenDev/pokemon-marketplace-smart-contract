# 🎮 Solana Pokémon cNFT Marketplace

A decentralized marketplace for **Pokémon-themed compressed NFTs (cNFTs)** on **Solana**, built using:

- [Metaplex Bubblegum](https://github.com/metaplex-foundation/mpl-bubblegum) for **compressed NFT minting & transfers**
- [Tensor cNFT Marketplace](https://github.com/tensor-foundation) smart contracts for **listings, bidding, and trading**

This project demonstrates how to mint, trade, and manage compressed NFTs efficiently with **low fees** and **high scalability**.

[![Twitter](https://img.shields.io/badge/Twitter-@toptrendev-black?style=for-the-badge&logo=twitter&logoColor=1DA1F2)](https://x.com/toptrendev)
[![Discord](https://img.shields.io/badge/Discord-toptrendev-black?style=for-the-badge&logo=discord&logoColor=5865F2)](https://discord.com/users/648385188774019072)
[![Telegram](https://img.shields.io/badge/Telegram-@TopTrenDev_66-black?style=for-the-badge&logo=telegram&logoColor=2CA5E0)](https://t.me/TopTrenDev_66)

---

## ✨ Features
- **Mint Pokémon cNFTs** using Metaplex Bubblegum
- **List, buy, and sell** Pokémon cNFTs via Tensor marketplace contract
- **Randomized minting** (optional: gacha/pack-opening style)
- **Fast & cheap transactions** thanks to Solana’s compression tech
- **On-chain metadata** with custom Pokémon attributes (type, rarity, stats)

---

## 📦 Program Overview

### Bubblegum Integration


  **Uses CPI calls into mpl-bubblegum for:**
  
  - Minting Pokémon cNFTs
  - Transferring Pokémon cNFTs between users

### Tensor Marketplace Integration


  **CPI calls into Tensor’s smart contract for:**
  
  - Listing Pokémon cNFTs
  - Canceling listings
  - Buying/Selling via buy_spl and list instructions

  
