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