/**
 * Deploy the name-registry contract to Asset Hub
 *
 * Usage: pnpm deploy [node-url]
 *
 * The deployed contract address is printed to stdout.
 */
import { createClient, Binary, FixedSizeBinary } from "polkadot-api";
import { getWsProvider } from "polkadot-api/ws-provider/web";
import { assetHub } from "@polkadot-api/descriptors";
import { readFileSync, writeFileSync } from "fs";
import { resolve } from "path";
import { prepareSigner } from "./signer";
import {
    GAS_LIMIT,
    STORAGE_DEPOSIT_LIMIT,
    DEFAULT_NODE_URL,
    DEFAULT_SIGNER,
} from "./constants";

const nodeUrl = process.argv[2] || DEFAULT_NODE_URL;
const signerName = process.argv[3] || DEFAULT_SIGNER;

// Path to the compiled contract bytecode
const BYTECODE_PATH = resolve(
    import.meta.dirname,
    "../../target/name-registry.release.polkavm",
);

async function main() {
    console.log(`Connecting to ${nodeUrl}...`);
    const client = createClient(getWsProvider(nodeUrl));
    const api = client.getTypedApi(assetHub);

    const signer = prepareSigner(signerName);
    console.log(`Using signer: ${signerName}`);

    // Read the contract bytecode
    const bytecode = readFileSync(BYTECODE_PATH);
    const code = Binary.fromBytes(bytecode);
    console.log(`Loaded bytecode: ${bytecode.length} bytes`);

    // Constructor takes no arguments, so data is empty
    const data = Binary.fromBytes(new Uint8Array(0));

    console.log("Deploying contract...");
    const result = await api.tx.Revive.instantiate_with_code({
        value: 0n,
        weight_limit: {
            ref_time: GAS_LIMIT.refTime,
            proof_size: GAS_LIMIT.proofSize,
        },
        storage_deposit_limit: STORAGE_DEPOSIT_LIMIT,
        code,
        data,
        salt: undefined,
    }).signAndSubmit(signer);

    // Find the Instantiated event to get the contract address
    const instantiatedEvent = result.events.find(
        (e) => e.type === "Revive" && e.value.type === "Instantiated",
    );

    if (!instantiatedEvent) {
        console.error("Contract instantiation failed - no Instantiated event");
        console.error("Events:", JSON.stringify(result.events, null, 2));
        process.exit(1);
    }

    const contractAddr = (
        instantiatedEvent.value.value as { contract: FixedSizeBinary<20> }
    ).contract.asHex();

    console.log("\n=== Contract Deployed ===");
    console.log(`Address: ${contractAddr}`);

    // Save the address to a file for easy reuse
    const addressFile = resolve(import.meta.dirname, "contract-address.txt");
    writeFileSync(addressFile, contractAddr);
    console.log(`Address saved to: ${addressFile}`);

    client.destroy();
}

main().catch((err) => {
    console.error("Deployment failed:", err);
    process.exit(1);
});
