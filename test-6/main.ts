import { ApiPromise, WsProvider } from "@polkadot/api";
import { ApiTypes } from "@polkadot/api/types";
import { Map } from "@polkadot/types";
import { CodeHash } from "@polkadot/types/interfaces";

async function getConnectChain() {
    const provider = new WsProvider('ws://127.0.0.1:9944')

    const api = await ApiPromise.create({ provider });

    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
    ]);
    console.log(`You are connected to \nchain:${chain}\nusing: ${nodeName} \nversion:v${nodeVersion}`);
}

async function traversEvents() {
    const api = await ApiPromise.create();
    api.query.system.events((events: Array<ApiTypes>) => {
        // console.log('---------------', event.length);
        console.log(`\nReceived ${events.length} events:`);
        // console.log(typeof events[0]);
        events.forEach((record: any) => {

            //  console.log(record.toHuman());
            const { event, phase } = record;
            if (event.method == 'SomethingStored') {
                const types = event.typeDef;
                // Show what we are busy with   
                console.log(`\t${event.section}:${event.method}:: (phase=${phase})`);
                console.log('---------------');
                console.log(`\t\t${event.meta}`);
                console.log('---------------');
                event.data.forEach((data: any, index: any) => {
                    console.log(`${types[index].type} : ${data}`);
                });
            }

        });
        //console.log(event.toHuman(), event.toRawType(),);
    });



}


function main() {
    //getConnectChain().catch(console.error);
    traversEvents().catch((error) => {
        console.error(error);
        process.exit(-1)
    });

}


main();