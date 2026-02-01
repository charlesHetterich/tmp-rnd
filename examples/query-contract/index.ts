/**
 * Test the name-registry contract by registering and querying names
 *
 * Usage: pnpm start [contract-address] [node-url]
 *
 * If no address is provided, reads from contract-address.txt (created by deploy.ts)
 */
import { createClient, SS58String } from "polkadot-api";
import { getWsProvider } from "polkadot-api/ws-provider/web";
import { contracts } from "@polkadot-api/descriptors";
import { createInkSdk } from "@polkadot-api/sdk-ink";
import { readFileSync, existsSync } from "fs";
import { resolve } from "path";
import { prepareSigner } from "./signer";
import {
    STORAGE_DEPOSIT_LIMIT,
    DEFAULT_NODE_URL,
    DEFAULT_SIGNER,
} from "./constants";

// Get contract address from CLI arg or file
function getContractAddress(): string {
    if (process.argv[2] && !process.argv[2].startsWith("ws://")) {
        return process.argv[2];
    }
    const addressFile = resolve(import.meta.dirname, "contract-address.txt");
    if (existsSync(addressFile)) {
        return readFileSync(addressFile, "utf-8").trim();
    }
    throw new Error(
        "No contract address provided. Run deploy.ts first or pass address as argument.",
    );
}

const contractAddress = getContractAddress();
const nodeUrl =
    process.argv[3] || process.argv[2]?.startsWith("ws://")
        ? process.argv[2]
        : DEFAULT_NODE_URL;

// Helper to encode a string as a bigint (U256)
function stringToU256(str: string): bigint {
    const encoder = new TextEncoder();
    const bytes = encoder.encode(str);
    if (bytes.length > 32) {
        throw new Error("String too long (max 32 bytes)");
    }
    let result = 0n;
    for (let i = 0; i < bytes.length; i++) {
        result = (result << 8n) | BigInt(bytes[i]);
    }
    // Left-pad to 256 bits (32 bytes)
    result = result << BigInt((32 - bytes.length) * 8);
    return result;
}

// Helper to decode a bigint (U256) to a string
function u256ToString(value: bigint): string {
    const bytes: number[] = [];
    for (let i = 0; i < 32; i++) {
        const byte = Number((value >> BigInt((31 - i) * 8)) & 0xffn);
        if (byte !== 0 || bytes.length > 0) {
            bytes.push(byte);
        }
    }
    // Trim trailing zeros
    while (bytes.length > 0 && bytes[bytes.length - 1] === 0) {
        bytes.pop();
    }
    return new TextDecoder().decode(new Uint8Array(bytes));
}

// Alice's SS58 address (for queries)
const ALICE_SS58 =
    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" as SS58String;

async function main() {
    console.log("=== Name Registry Contract Test ===\n");
    console.log(`Contract: ${contractAddress}`);
    console.log(`Node: ${nodeUrl}`);

    // Connect to Asset Hub
    console.log("\nConnecting...");
    const client = createClient(getWsProvider(nodeUrl));

    // Set up the typed contract SDK
    const inkSdk = createInkSdk(client);
    const nameRegistry = inkSdk.getContract(
        contracts.nameRegistry,
        contractAddress,
    );

    const signer = prepareSigner(DEFAULT_SIGNER);
    console.log(`Signer: ${DEFAULT_SIGNER}`);

    // Test 1: Query myName (should be 0 initially)
    console.log("\n--- Test 1: Query myName (before register) ---");
    const myNameBefore = await nameRegistry.query("myName", {
        origin: ALICE_SS58,
        data: {},
    });
    if (!myNameBefore.success) {
        console.error("Query failed:", myNameBefore.value);
        process.exit(1);
    }
    console.log(`myName raw response:`, myNameBefore.value.response);
    const nameStrBefore = u256ToString(myNameBefore.value.response);
    console.log(
        `Current name: "${nameStrBefore}" (raw: ${myNameBefore.value.response})`,
    );

    // Test 2: Register a name
    const testName = "Alice";
    const nameAsU256 = stringToU256(testName);
    console.log(`\n--- Test 2: Register name "${testName}" ---`);
    console.log(`Name as U256: ${nameAsU256}`);

    const registerTx = nameRegistry.send("register", {
        data: { name: nameAsU256 },
        gasLimit: { ref_time: 500_000_000_000n, proof_size: 2_000_000n },
        storageDepositLimit: STORAGE_DEPOSIT_LIMIT,
    });

    const registerResult = await registerTx.signAndSubmit(signer);
    console.log(`Register tx finalized in block: ${registerResult.block.hash}`);

    // Test 3: Query myName again (should now return our name)
    console.log("\n--- Test 3: Query myName (after register) ---");
    const myNameAfter = await nameRegistry.query("myName", {
        origin: ALICE_SS58,
        data: {},
    });
    if (!myNameAfter.success) {
        console.error("Query failed:", myNameAfter.value);
        process.exit(1);
    }
    console.log(`myName raw response:`, myNameAfter.value.response);
    const nameStrAfter = u256ToString(myNameAfter.value.response);
    console.log(`Current name: "${nameStrAfter}"`);

    // Test 4: Lookup by address (Alice's H160 address)
    // Note: We need to convert Alice's public key to H160
    // For now, let's query using origin to get the correct address mapping
    console.log("\n--- Test 4: Query lookup (by address) ---");
    // Alice's H160 derived from her SS58 (this is chain-specific)
    // We'll just demonstrate the query structure
    const lookupResult = await nameRegistry.query("lookup", {
        origin: ALICE_SS58,
        data: { addr: "0x0000000000000000000000000000000000000001" }, // placeholder
    });
    if (!lookupResult.success) {
        console.error("Query failed:", lookupResult.value);
        process.exit(1);
    }
    console.log(`lookup raw response:`, lookupResult.value.response);
    const lookupName = u256ToString(lookupResult.value.response);
    console.log(`Lookup result: "${lookupName}"`);

    console.log("\n=== Tests Complete ===");
    client.destroy();
}

main().catch((err) => {
    console.error("Error:", err);
    process.exit(1);
});
