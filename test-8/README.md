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

## ERC20

- build 
  ![build](/img/build.png)
- transfer
  ![tr](img/upload.png)
  ![tr](img/new.png)
  ![tr](img/call.png)
  ![tr](img/trs.png)
  ![tr](img/balance.png)

- node log
  ![log](img/log.png)