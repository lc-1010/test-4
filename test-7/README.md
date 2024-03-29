# Substrate
 
- [Substrate](#substrate)
  - [1 回顾-单元测试](#1-回顾-单元测试)
    - [2 cargo test](#2-cargo-test)
  - [3 Kitty 模块](#3-kitty-模块)
    - [买卖](#买卖)
    - [全部方法](#全部方法)
  - [4 kitty lib 单元测试](#4-kitty-lib-单元测试)
    - [完成ERROR 测试 18项目](#完成error-测试-18项目)
      - [cargo clippy](#cargo-clippy)
    - [回滚问题 现在已经默认又了 不用特别加宏](#回滚问题-现在已经默认又了-不用特别加宏)
  - [kitty 前端页面 暂时不会�‍♂️�](#kitty-前端页面-暂时不会️)
      - [todo](#todo)
    - [web](#web)
    - [](#)
  - [4 offchain](#4-offchain)
      - [offchain 组件](#offchain-组件)
    - [offchain实践](#offchain实践)
      - [Indexing](#indexing)
      - [localStorageGet](#localstorageget)

---- 

## 1 回顾-单元测试

- [x] 创建存证测试
- [x] 撤销存证
- [x] 转移存证

### 2 cargo test

- 参数 -p 模块

- 脚本
```shell
 cargo test --package pallet-poe --lib -- tests --nocapture

    Finished test [unoptimized + debuginfo] target(s) in 0.21s
     Running unittests src/lib.rs (target/debug/deps/pallet_poe-f56fe88d88f715e4)

running 12 tests
test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
test tests::create_claim ... ok
test tests::create_claim_already_exist ... ok
test tests::claim_transfer_not_owner ... ok
test tests::claim_transfer_claim_toolong ... ok
test tests::claim_transfer_not_exist ... ok
test tests::claim_transfer ... ok
test tests::create_claim_toolong ... ok
test tests::claim_revoke_not_exist ... ok
test tests::revoke_claim_toolong ... ok
test tests::revoke_claim ... ok
test tests::revoke_not_claim_owner ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
![img](kitty-1/2022-09-0223.34.59.png)

## 3 Kitty 模块

- 增加买卖 extrinsic
- 不再pallet中指定，在runtime中绑定
- test 模块
- 引入 balances 内方法，质押token，购买时支付token

- 参考 ![treasury](https://github.com/paritytech/substrate/tree/master/frame/treasury)

### 买卖

```shell
cargo build --release
   Compiling node-template-runtime v4.0.0-dev (/test-4/test-7/substrate-node-template-polkadot-v0.9.25/runtime)
   Compiling node-template v4.0.0-dev (/test-4/test-7/substrate-node-template-polkadot-v0.9.25/node)
    Finished release [optimized] target(s) in 1m 52s

 ./target/release/node-template --dev
2022-09-10 20:53:39 Substrate Node
2022-09-10 20:53:39 ✌️  version 4.0.0-dev-863e97f8f47
2022-09-10 20:53:39 ❤️  by Substrate DevHub <https://github.com/substrate-developer-hub>, 2017-2022
2022-09-10 20:53:39 📋 Chain specification: Development
2022-09-10 20:53:39 🏷  Node name: cowardly-agreement-6409
2022-09-10 20:53:39 👤 Role: AUTHORITY
2022-09-10 20:53:39 💾 Database: RocksDb at /var/folders/td/bs9n_nsj4vv2vdv16t_psxwm0000gn/T/substratedmbX8o/chains/dev/db/full
2022-09-10 20:53:39 ⛓  Native runtime: node-template-100 (node-template-1.tx1.au1)
2022-09-10 20:53:39 🔨 Initializing Genesis block/state (state: 0xc610…ac23, header-hash: 0x55be…e860)
2022-09-10 20:53:39 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.
2022-09-10 20:53:39 Using default protocol ID "sup" because none is configured in the chain specs
2022-09-10 20:53:39 🏷  Local node identity is: 12D3KooWSqDdqYKgJSmT8X5zv274kdepuAW5zXteT6Pk4AcTpHLM
2022-09-10 20:53:39 💻 Operating system: macos
2022-09-10 20:53:39 💻 CPU architecture: aarch64
2022-09-10 20:53:39 📦 Highest known block at #0
2022-09-10 20:53:39 〽️ Prometheus exporter started at 127.0.0.1:9615
2022-09-10 20:53:39 Running JSON-RPC HTTP server: addr=127.0.0.1:9933, allowed origins=None
2022-09-10 20:53:39 Running JSON-RPC WS server: addr=127.0.0.1:9944, allowed origins=None
2022-09-10 20:53:42 🙌 Starting consensus session on top of parent 0x55be31b84f1476f78eb787f7b26d0c8fcdf913d3fd3d724357af4a96bddee860
2022-09-10 20:53:42 🎁 Prepared block for proposing at 1 (2 ms) [hash: 0x609ddf3764138509d4cbdf6d63b4ef0fc2a2aab5bd8752ba743f632977c150fe; parent_hash: 0x55be…e860; extrinsics (1): [0x5740…b8e9]]
```
### 全部方法
- [全部方法](https://github.com/lc-1010/test-4/blob/main/test-7/substrate-node-template-polkadot-v0.9.25/pallets/kitties/src/lib.rs)

   ![img](kitty-1/all-fun-2022-09-1021.11.44.png)
- [创建](https://github.com/lc-1010/test-4/blob/dfc3cc3e66b89dc92a46e865246bb8c1cf830c9b/test-7/substrate-node-template-polkadot-v0.9.25/pallets/kitties/src/lib.rs#L115)

   ![img](kitty-1/creat-2022-09-1020.56.13.png)
- [繁育](https://github.com/lc-1010/test-4/blob/dfc3cc3e66b89dc92a46e865246bb8c1cf830c9b/test-7/substrate-node-template-polkadot-v0.9.25/pallets/kitties/src/lib.rs#L129)
   ![img](kitty-1/breed-2022-09-1020.57.02.png)
- [转移](https://github.com/lc-1010/test-4/blob/dfc3cc3e66b89dc92a46e865246bb8c1cf830c9b/test-7/substrate-node-template-polkadot-v0.9.25/pallets/kitties/src/lib.rs#L162)
   ![img](kitty-1/transfer-2022-09-1020.57.32.png)
   ![img](kitty-1/transfer2022-09-1020.57.25.png)
- [买](https://github.com/lc-1010/test-4/blob/dfc3cc3e66b89dc92a46e865246bb8c1cf830c9b/test-7/substrate-node-template-polkadot-v0.9.25/pallets/kitties/src/lib.rs#L189)
   ![img](kitty-1/buy-2022-09-1021.30.25.png)
- [卖](https://github.com/lc-1010/test-4/blob/dfc3cc3e66b89dc92a46e865246bb8c1cf830c9b/test-7/substrate-node-template-polkadot-v0.9.25/pallets/kitties/src/lib.rs#L175)
   ![img](kitty-1/sell-2022-09-1020.58.39.png)
   ![img](kitty-1/sell-2022-09-1020.59.16.png)
- 查询
   ![img](kitty-1/check-2022-09-1021.00.54.png)

- [检测账户](https://github.com/lc-1010/test-4/blob/dfc3cc3e66b89dc92a46e865246bb8c1cf830c9b/test-7/substrate-node-template-polkadot-v0.9.25/pallets/kitties/src/lib.rs#L261)

## 4 kitty lib 单元测试 
   
测试代码编写，修改了原始方法错误信息以及逻辑错误。
   - assert_noop！ 验证了 是否修改了存储-在lib 中逻辑错误 造成了storage 改变 先检查后修改，没有回滚 [storage 改变 ](https://stackoverflow.com/questions/66984081/assert-noop-doesnt-pass-the-test)
   -  buy,sell缺少校验所有者
   - pallet_balances::GenesisConfig 和  Balances::set_balance(Origin::root(), 6, 1, 0)
   - new_test_ext().execute_with(||{}) 缺失报错

### 完成ERROR 测试 18项目
```
   Compiling pallet-kitties v4.0.0-dev (/test-4/test-7/substrate-node-template-polkadot-v0.9.25/pallets/kitties)
    Finished test [unoptimized + debuginfo] target(s) in 2.20s
     Running unittests src/lib.rs (target/debug/deps/pallet_kitties-4bd1ce8350722db0)

running 18 tests
test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
test tests::create_kitty_error_moneynotenough ... ok
test tests::bread_kitty ... ok
test tests::buy ... ok
test tests::bread_kitty_error_samekittyid ... ok
test tests::create_kitty ... ok
test tests::buy_error_money ... ok
test tests::bread_kitty_error_invaidkittyid ... ok
test tests::create_with_error_kittyindexoverflow ... ok
test tests::sell ... ok
test tests::buy_error_owner ... ok
test tests::sell_error_money ... ok
test tests::transfer ... ok
test tests::sell_error_owner ... ok
test tests::transfer_err_invaidkittyid ... ok
test tests::transfer_err_sameaccount ... ok
test tests::transfer_err_moneynotenough ... ok
test tests::transfer_error_notkittywwner ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

 *  终端将被任务重用，按任意键关闭。 
 ``` 
#### cargo clippy
```
cargo clippy
    Checking pallet-randomness-collective-flip v4.0.0-dev (https://github.com/paritytech/substrate.git?branch=polkadot-v0.9.25#3348e144)
    Checking pallet-balances v4.0.0-dev (https://github.com/paritytech/substrate.git?branch=polkadot-v0.9.25#3348e144)
    Checking pallet-kitties v4.0.0-dev (/test-4/test-7/substrate-node-template-polkadot-v0.9.25/pallets/kitties)
warning: redundant clone
   --> pallets/kitties/src/lib.rs:239:34
    |
239 |             let who = ensure_signed(origin.clone())?;
    |                                           ^^^^^^^^ help: remove this
    |
    = note: `#[warn(clippy::redundant_clone)]` on by default
note: this value is dropped without further use
   --> pallets/kitties/src/lib.rs:239:28
    |
239 |             let who = ensure_signed(origin.clone())?;
    |                                     ^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone

warning: this match could be written as a `let` statement
   --> pallets/kitties/src/lib.rs:275:4
    |
275 | /             match Self::next_kitty_id() {
276 | |                 val => {
277 | |                     ensure!(val != T::KittyIndex::max_value(), Error::<T>::KittyIndexOverFlow);
278 | |                     Ok(val)
279 | |                 },
280 | |             }
    | |_____________^
    |
    = note: `#[warn(clippy::match_single_binding)]` on by default
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#match_single_binding
help: consider using `let` statement
    |
275 ~             let val = Self::next_kitty_id();
276 +    {
277 ~                        ensure!(val != T::KittyIndex::max_value(), Error::<T>::KittyIndexOverFlow);
278 ~                        Ok(val)
279 ~                    }
    |

warning: `pallet-kitties` (lib) generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 2.03s
```
### 回滚问题 现在已经默认又了 不用特别加宏
#[transactional]

use of deprecated macro `transactional`: This is now the default behaviour for all extrinsics.
`#[warn(deprecated)]` on by default


## kitty 前端页面 暂时不会�‍♂️�
 yarn cache clean --all
YARN_CHECKSUM_BEHAVIOR=update yarn

#### todo
- [ ] 创建
- [ ] 展示，属于谁
- [ ] 转让 

###  [web](./front-web/) 
### 

## 4 offchain 
### offchain 组件
- offchain worker  ==>read==> ruintime storage 
- offchain storage ===》 runtime storage 
- offchain indexing
node 通过api调用链上和链下逻辑

链下逻辑处理 
- offchain workr 可以直接读链上数据，不能写
- 链上代码可以向offchain Storage 中写数据，但不能读
- 外层node 和rpc 可以直接读链上存储的数据，和offchain Storage 中的数据
- 外层node 和rpc 可以直接向offchain Storage 中写数据，但不能直接向链上存储写数据，必须通过发生交易更改链上状态

### offchain实践

#### Indexing
- [x] 1.在 Offchain Worker 中，使用 Offchain Indexing 特性实现从链上向 Offchain Storage 中写入数据

- [read IndexingData code ](https://github.com/lc-1010/test-4/blob/main/test-7/substrate-node-template-polkadot-v0.9.25/pallets/offchain-indexing/src/lib.rs#L121)
   - ![img](./offchain-img/local-data.png)
  
#### localStorageGet
- [x] 2.使用 js sdk 从浏览器 frontend 获取到前面写入 Offchain Storage 的数据  
   - 简单通过  `api.rpc.offchain.localStorageGet` 函数调用获取数据,然后将btye 数据转为 字符串，比较麻烦 �
   - [api.rpc.offchain.localStorageGet](https://github.com/lc-1010/test-4/blob/main/test-7/offchain-js/main.ts#L19)
     - ![img](./offchain-img/js-api-rpc-value.png)
  
- [x] 3.回答链上随机数（如前面Kitties示例中）与链下随机数的区别
   
   链上数据中的随机数使用伪随机数，容易被猜测出算法种子，导致不安全的问题。
   链下随机数相对更安全。
   - https://happypeter.github.io/binfo/chain-random
   - https://learnblockchain.cn/article/1083

- [ ] 4.（可选）在 Offchain Worker 中，解决向链上发起不签名请求时剩下的那个错误。参考：https://github.com/paritytech/substrate/blob/master/frame/examples/offchain-worker/src/lib.rs 
   - todo 
  
- [x] 5.（可选）构思一个应用场景，描述如何使用 Offchain Features 三大组件去实现它
  
  - 有一个需要大量计算，或者是交易，才能得出的结果，但是使用链上数据作为一个重要的参数，并最终写会链上。或者是通过rpc 调用方式将两个数据打通，这样使用链上数据。
  
  
- [ ] 6.（可选）如果有时间，可以实现一个上述原型

