import { ApiPromise, WsProvider } from "@polkadot/api";
import { Int, u32 } from "@polkadot/types";
import { Events } from "@polkadot/types/metadata/decorate/types";

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
    api.query.system.events((events: any) => {

        console.log(`\nReceived   ${events} events: \n`);
        events.forEach((record: any) => {
            const { event, phase } = record;
            const types = event.typeDef;
            console.log(`\t${event.section}: ${event.method}:: (phase = ${phase.toString()})`);
            console.log(`\t\t${event} `);
            event.data.forEach((data: any, index: any) => {
                console.log(`\n\r{types[index].type}:${data.toString()} `);
            });

        });

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