import { ApiPromise, WsProvider } from "@polkadot/api";
import { u64, Vec } from "@polkadot/types";
import type { EventRecord } from '@polkadot/types/interfaces';

async function getConnectChain() {
    const provider = new WsProvider('ws://127.0.0.1:9944')

    const api = await ApiPromise.create({ provider });

    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
    ]);
    console.log(`You are connected to \nchain:${chain}\nusing: ${nodeName} \nversion:v${nodeVersion}`);
    const ONCHAIN_TX_KEY = 'my_pallet::indexing1'
    //保存的key 前缀 IndexingData key is Ok("my_pallet::indexing1/\u{2}\0\0\0")
    let key = `${ONCHAIN_TX_KEY}/\u{3}\0\0\0`
    let data = api.rpc.offchain.localStorageGet('PERSISTENT', key)
    let da= !(await data).isNone ? (await data).unwrap() : '-1'
    console.log(`You Data:${da}`)
    const text = da.slice(1, 2000)
      const rawData = byteToString(text)
      console.log(rawData)
}
function byteToString(arr: string | Uint8Array) {
    if (typeof arr === 'string') {
      return arr;
    }
    var str = '',
      _arr = arr;
    for (var i = 0; i < _arr.length; i++) {
      var one = _arr[i].toString(2),
        v = one.match(/^1+?(?=0)/);
      if (v && one.length == 8) {
        var bytesLength = v[0].length;
        var store = _arr[i].toString(2).slice(7 - bytesLength);
        for (var st = 1; st < bytesLength; st++) {
          store += _arr[st + i].toString(2).slice(2);
        }
        str += String.fromCharCode(parseInt(store, 2));
        i += bytesLength - 1;
      } else {
        str += String.fromCharCode(_arr[i]);
      }
    }
    return str;
  }

function main() {
    getConnectChain().catch(console.error);
   

}


main();

/*
>> npx ts-node main.ts
You are connected to
chain:Development
using: Substrate Node
version:v4.0.0-dev-ddc90c1913d
You Data:0x347375626d69745f6e756d6265720200000000000000
*/