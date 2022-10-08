# Ink! 开发智能合约

## 安装
``` bash
  git clone https://github.com/paritytech/substrate-contracts-node.git
  git checkout v0.20.0
  ls
  cd substrate-contracts-node 

  rustup component add rust-src --toolchain nightly
  rustup target add wasm32-unknown-unknown --toolchain nightly
  cargo install dylint-link
  cargo install cargo-contract --force
 cargo contract --help
```

### 存储 

- #[ink(storage)]
- #[ink(constructor)]
- #[ink(message)]
- #[ink(event)]

### 版本 2.0.0
```
cargo install --force --locked cargo-contract --vers 2.0.0-alpha.1

cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --tag v0.20.0 --force --locked
```
