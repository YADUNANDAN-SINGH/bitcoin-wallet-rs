<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=F7931A&height=200&section=header&text=bitcoin-wallet-rs&fontSize=50&fontColor=ffffff&animation=fadeIn&fontAlignY=38&desc=A%20Bitcoin%20wallet%20written%20in%20Rust%20from%20scratch&descAlignY=60&descAlign=50" />
</p>

<p align="center">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&size=18&pause=1000&color=F7931A&center=true&vCenter=true&width=500&lines=Key+generation+%E2%9C%93;Wallet+persistence+%E2%9C%93;UTXO+discovery+%E2%9C%93;Unsigned+transaction+construction+%E2%9C%93;Signing+coming+soon...;Broadcasting+coming+soon...;Confirmation+coming+soon..." alt="Typing SVG" />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.75+-orange?style=flat-square&logo=rust" />
  <img src="https://img.shields.io/badge/network-signet-blue?style=flat-square&logo=bitcoin" />
  <img src="https://img.shields.io/badge/status-WIP-yellow?style=flat-square" />
  <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" />
</p>

---

# bitcoin-wallet-rs

A Bitcoin wallet written in Rust from scratch. Targets signet.

🚧 **Work in progress.** Currently implements: key generation, wallet persistence, UTXO discovery, and unsigned transaction construction. Signing, broadcasting, and confirmation coming soon.

Built on [`rust-bitcoin`](https://github.com/rust-bitcoin/rust-bitcoin) and [`secp256k1`](https://github.com/rust-bitcoin/rust-secp256k1).

---

## Features

| Feature | Status |
|---|---|
| Key generation | ✅ Done |
| Wallet persistence | ✅ Done |
| UTXO discovery | ✅ Done |
| Unsigned transaction construction | ✅ Done |
| Transaction signing | 🔜 Soon |
| Transaction broadcasting | 🔜 Soon |
| Confirmation tracking | 🔜 Soon |

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

## Network

This wallet targets **signet** — a test network with signed blocks, making it ideal for development without real funds.

<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=F7931A&height=100&section=footer" />
</p>
