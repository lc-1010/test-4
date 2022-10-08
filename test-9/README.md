# Benchmarks

>> cargo build --release --features runtime-benchmarks

## 生成weight.rs 文件
```sh

./target/release/node-template benchmark pallet --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_poe --extrinsic "*"  --steps 50 --repeat 20 --output ./pallets/poe/src/weights.rs --template .maintain/frame-weight-template.hbs
Pallet: "pallet_poe", Extrinsic: "create_claim", Lowest values: [], Highest values: [], Steps: 50, Repeat: 20
Raw Storage Info
========
Storage: PoeModule Proofs (r:1 w:1)

Median Slopes Analysis
========
-- Extrinsic Time --

Model:
Time ~=       93
    + d        0
              µs

Reads = 1 + (0 * d)
Writes = 1 + (0 * d)

Min Squares Analysis
========
-- Extrinsic Time --

Data points distribution:
    d   mean µs  sigma µs       %
    0      88.7     12.17   13.7%
   10      98.9     5.243    5.3%
   20      87.6     3.878    4.4%
   30      68.8     14.39   20.9%
   40      81.9      0.83    1.0%
   50      86.9     5.185    5.9%
   60      89.3     3.579    4.0%
   70      88.7     8.683    9.7%
   80      79.1     23.63   29.8%
   90     110.5     10.07    9.1%
  100        97     7.784    8.0%
  110     101.7      5.02    4.9%
  120      53.4     8.834   16.5%
  130     103.1     5.521    5.3%
  140     103.9     10.22    9.8%
  150      89.4       2.2    2.4%
  160     102.2     2.856    2.7%
  170      64.6     17.18   26.5%
  180     102.4     4.694    4.5%
  190      88.8     1.166    1.3%
  200      95.8     4.833    5.0%
  210     105.3     6.986    6.6%
  220     112.4     9.013    8.0%
  230     113.4     9.478    8.3%
  240     114.5      10.8    9.4%
  250      89.8     1.661    1.8%
  260     100.1     2.507    2.5%
  270     101.3     2.238    2.2%
  280     101.1      8.96    8.8%
  290      89.3     1.486    1.6%
  300      73.8     15.69   21.2%
  310      92.7     7.321    7.8%
  320      49.1       0.3    0.6%
  330      49.1       0.3    0.6%
  340      74.5      19.7   26.4%
  350      90.3     2.282    2.5%
  360      72.5     16.53   22.8%
  370      91.3     3.163    3.4%
  380      60.3     11.53   19.1%
  390      91.7     1.004    1.0%
  400      92.6     1.428    1.5%
  410      91.7       1.9    2.0%
  420      84.8     16.02   18.8%
  430      93.8     0.748    0.7%
  440      93.9     2.426    2.5%
  450      50.2       0.4    0.7%
  460      93.9     2.256    2.4%
  470     101.9     6.268    6.1%
  480      97.8     3.458    3.5%
  490      97.7     6.213    6.3%
  500      95.6     1.019    1.0%
  510      95.6     1.743    1.8%

Quality and confidence:
param     error
d         0.005

Model:
Time ~=    91.06
    + d        0
              µs

Reads = 1 + (0 * d)
Writes = 1 + (0 * d)
```

## benchmarks 

- [weights.rs](./substrate-v0.9.25-benchmark/pallets/poe/src/weights.rs)
- [create_claim](./substrate-v0.9.25-benchmark/pallets/poe/src/lib.rs#L65), 
- [revoke_claim](./substrate-v0.9.25-benchmark/pallets/poe/src/lib.rs#L87), 
- [transfer_claim](./substrate-v0.9.25-benchmark/pallets/poe/src/lib.rs#L102)

## Chain Spec

```sh
git clone https://github.com/paritytech/substrate.git
cd substrate
cargo +nightly build --package subkey --release
./target/release/subkey --help
```

for i in 1 2 3 4; do for j in babe; do ./target/release/subkey  --sr25519 inspect "$SECRET//$i//$j"; done; done

for i in 1 2 3 4; do for j in stash controller; do ./target/release/subkey inspect "$SECRET//$i//$j"; done; done
Secret Key URI `//1//stash` is account:
  Network ID:        substrate
 Secret seed:       0x76ca099086ceb21acf24b0879b32ac96f1cae82ef7b5ad49d8c8326dce6e0701
  Public key (hex):  0x62692ff7965729b21a10862da513beb42b7da54f321e35e2147983e1e53dc935
  Account ID:        0x62692ff7965729b21a10862da513beb42b7da54f321e35e2147983e1e53dc935
  Public key (SS58): 5EHjoBVGQm4CNbdoeaozpAKGp9TTrfRuADvQ65kyiABZhYPk
  SS58 Address:      5EHjoBVGQm4CNbdoeaozpAKGp9TTrfRuADvQ65kyiABZhYPk
Secret Key URI `//1//controller` is account:
  Network ID:        substrate
 Secret seed:       0x227eeea613bb3d95ad2bb16195436d68f46840ce2b4292afe4f4e440ab643639
  Public key (hex):  0x3cd09eecf6faa579ff49a5bb8175c02244da1151cfa75b8b3fc9dcb15b4b281d
  Account ID:        0x3cd09eecf6faa579ff49a5bb8175c02244da1151cfa75b8b3fc9dcb15b4b281d
  Public key (SS58): 5DSShm3qptXjE5aK7aUoVCQ7ScgCwt8wbH7MzgNwtRg4FPJZ
  SS58 Address:      5DSShm3qptXjE5aK7aUoVCQ7ScgCwt8wbH7MzgNwtRg4FPJZ
Secret Key URI `//2//stash` is account:
  Network ID:        substrate
 Secret seed:       0xcdbfa0952070f3e84d47e66e58662d5fb03493ed504279c663509827f233a120
  Public key (hex):  0xa83f9b156daa23ac07dd3361514d1b9f1674904d35ce8ab422bc8e1e12dac70b
  Account ID:        0xa83f9b156daa23ac07dd3361514d1b9f1674904d35ce8ab422bc8e1e12dac70b
  Public key (SS58): 5FsJoADMCmUQqmprSSqnyLPj7KAz7Kym6AzWbXMyEza7A5XH
  SS58 Address:      5FsJoADMCmUQqmprSSqnyLPj7KAz7Kym6AzWbXMyEza7A5XH
Secret Key URI `//2//controller` is account:
  Network ID:        substrate
 Secret seed:       0xa224f6e5a1d971019c3d3d012a6980ff6cd20686a345d121a2373029ef014898
  Public key (hex):  0xb819d8c01cbc46e23d9b79f7654f704a828fa1946bc8a97f56889daade1ced4e
  Account ID:        0xb819d8c01cbc46e23d9b79f7654f704a828fa1946bc8a97f56889daade1ced4e
  Public key (SS58): 5GE6M2FBBChfGfatFvRmWSgJrvSuxVYB2HNA13Fb5EFMpjst
  SS58 Address:      5GE6M2FBBChfGfatFvRmWSgJrvSuxVYB2HNA13Fb5EFMpjst
Secret Key URI `//3//stash` is account:
  Network ID:        substrate
 Secret seed:       0xe8209a08c949f06bf1dc6bd52c5056f949a42a998f0f3d9441ed907e6d1e0552
  Public key (hex):  0x5e25b78d7ef73fb03c48b5550c7762f2fffaff54ef6cac0d670157cf2ba18563
  Account ID:        0x5e25b78d7ef73fb03c48b5550c7762f2fffaff54ef6cac0d670157cf2ba18563
  Public key (SS58): 5EC9ZatAJNcG1zrbkuJJjwfe4k2Dbs3XmDvk4MBjKanw6tpL
  SS58 Address:      5EC9ZatAJNcG1zrbkuJJjwfe4k2Dbs3XmDvk4MBjKanw6tpL
Secret Key URI `//3//controller` is account:
  Network ID:        substrate
 Secret seed:       0x59e78df01d000ebc416cac300d44beb36327d2680a296d78e06c140156d8f293
  Public key (hex):  0x1897739a555a3ffc548045b2d3580510e9d30e4529d7b92bc35da4421200d160
  Account ID:        0x1897739a555a3ffc548045b2d3580510e9d30e4529d7b92bc35da4421200d160
  Public key (SS58): 5CcwzmrQg71nAQX8XpFKXojxF7GUHY5C74c4HeXdxa5uVFxK
  SS58 Address:      5CcwzmrQg71nAQX8XpFKXojxF7GUHY5C74c4HeXdxa5uVFxK
Secret Key URI `//4//stash` is account:
  Network ID:        substrate
 Secret seed:       0xc3d1ca50b4ce359d7df48b41499e637e71173061a01c3e2caf5025c6f4feda52
  Public key (hex):  0x9abfacaa81504b72ed3d0e4379e6604320b394cb5dc75f89bac64fc0798a901a
  Account ID:        0x9abfacaa81504b72ed3d0e4379e6604320b394cb5dc75f89bac64fc0798a901a
  Public key (SS58): 5FZcArDEk2ddwCRDVn6NzuL68X6cAcoqAxZ2kkLLMjG3wBgb
  SS58 Address:      5FZcArDEk2ddwCRDVn6NzuL68X6cAcoqAxZ2kkLLMjG3wBgb
Secret Key URI `//4//controller` is account:
  Network ID:        substrate
 Secret seed:       0x05d1f362942ee57772f3547d735dcf0f7d0d138f0406f1994d12ae620d0a673c
  Public key (hex):  0x825ca97d686833d181f76235eab34d3c415947a3363f559db3d966616291cb53
  Account ID:        0x825ca97d686833d181f76235eab34d3c415947a3363f559db3d966616291cb53
  Public key (SS58): 5F1dcFSJkE9RweJqsjKbH8VFdpo1Pibo6s5m4UxWs1zqH59E
  SS58 Address:      5F1dcFSJkE9RweJqsjKbH8VFdpo1Pibo6s5m4UxWs1zqH59E

  ======
