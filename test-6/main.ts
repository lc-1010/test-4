import { ApiPromise, WsProvider } from "@polkadot/api";
import { Vec } from "@polkadot/types";
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
}

async function traversEvents() {
    const api = await ApiPromise.create();

    // Subscribe to system events via storage
    api.query.system.events((events: Vec<EventRecord>) => {
        console.log(`\nReceived ${events.length} events:`);

        // Loop through the Vec<EventRecord>
        events.forEach((record: EventRecord) => {
            // Extract the phase, event and the event types
            const { event, phase } = record;
            const types = event.typeDef;

            // Show what we are busy with
            console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);
            console.log(`\t\t${event.meta.docs.toString()}`);

            // Loop through each of the parameters, displaying the type and data
            event.data.forEach((data, index) => {
                console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
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