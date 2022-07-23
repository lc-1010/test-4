# POE

https://github.com/lc-1010/substrate-node-template/tree/my-poe 

## 存证 撤销 转移
- function 
    
    ![myPoe 存证 撤销 转移 ](2022-07-2316.01.08.png) 
- [create_claim-code](https://github.com/lc-1010/substrate-node-template/blob/my-poe/pallets/poe/src/lib.rs#L49)

### 创建
- create
![create_claim](2022-07-2316.03.28.png)
- check
![check_claim](2022-07-2316.01.56.png)

### 撤销
- [revoek_claim](https://github.com/lc-1010/substrate-node-template/blob/my-poe/pallets/poe/src/lib.rs#L62)

    ![revoke](2022-07-2316.02.21.png)
- check 
    ![check](2022-07-2316.02.57.png)

### 转移

- [move_claim_code](https://github.com/lc-1010/substrate-node-template/blob/my-poe/pallets/poe/src/lib.rs#L77)

    ![move](2022-07-2316.03.48.png)
- check
![check](2022-07-2316.04.10.png)