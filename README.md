# NEAR-Tools

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.86.0%2B-orange.svg)](https://www.rust-lang.org/)

> **Zero-dependency CLI tool for creating NEAR smart contracts in seconds**

NEAR-Tools is a simple command-line tool that generates a complete, working NEAR smart contract project with all the correct settings. No more dependency hell, no more version conflicts, no more hours of setup.

## 🚀 Quick Start

```bash
# Install near-tools
cargo install near-tools

# Create your first NEAR contract
near-tools new my-first-contract

# Enter the project
cd my-first-contract

# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test