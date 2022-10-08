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

<<<<<<< HEAD
### 存储 
=======
### 存储
>>>>>>> a5b97fbfb4d00467261bc4d6635b6732693c3398

- #[ink(storage)]
- #[ink(constructor)]
- #[ink(message)]
- #[ink(event)]

<<<<<<< HEAD
### 版本 2.0.0
```
cargo install --force --locked cargo-contract --vers 2.0.0-alpha.1

cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --tag v0.20.0 --force --locked
```
=======
## ERC20

- build
  ![bu](img/build.png)

[initialize_contract 打印log](https://github.com/lc-1010/test-4/blob/main/test-8/erc20/lib.rs#L61)

```rust

 ink_lang::utils::initialize_contract(|contract: &mut Self| {

  ....

ink_env::debug_println!(
                    "打印log constructor total_supply{}: initialize_contract ",
                    total_supply
```

- transfer
  ![tr](img/upload.png)
  ![tr](img/new.png)
  ![tr](img/call.png)
  ![tr](img/trs.png)
  ![tr](img/balance.png)

- node log
  ![log](img/log.png)
>>>>>>> a5b97fbfb4d00467261bc4d6635b6732693c3398
