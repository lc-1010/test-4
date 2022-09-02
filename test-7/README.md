# 回顾-单元测试

- [x] 创建存证测试
- [x] 撤销存证
- [ ] 转移存证

## cargo test

- 参数 -p 模块

- 脚本
```shell
cargo test -p pallet-poe
    Finished test [unoptimized + debuginfo] target(s) in 0.76s
     Running unittests src/lib.rs (target/debug/deps/pallet_poe-541529f86a8e0aa9)

running 6 tests
test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
test tests::create_claim_already_exist ... ok
test tests::not_claim_owner ... ok
test tests::claim_not_exist ... ok
test tests::create_claim ... ok
test tests::revoke_claim ... ok


test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests pallet-poe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
