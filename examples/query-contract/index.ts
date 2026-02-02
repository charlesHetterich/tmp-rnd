/**
 * Test the name-registry contract
 * Usage: pnpm start [contract-address] [node-url]
 */
import { createClient, SS58String } from "polkadot-api";
import { getWsProvider } from "polkadot-api/ws-provider/web";
import { contracts } from "@polkadot-api/descriptors";
import { createInkSdk } from "@polkadot-api/sdk-ink";
import { readFileSync, existsSync } from "fs";
import { resolve } from "path";
import { prepareSigner } from "./signer";
import { GAS_LIMIT, STORAGE_DEPOSIT_LIMIT, DEFAULT_NODE_URL } from "./constants";

const ALICE: SS58String = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

const contractAddress =
    process.argv[2] ||
    (existsSync(resolve(import.meta.dirname, "contract-address.txt"))
        ? readFileSync(
              resolve(import.meta.dirname, "contract-address.txt"),
              "utf-8",
          ).trim()
        : (() => {
              throw new Error(
                  "No contract address. Run deploy.ts first or pass as arg.",
              );
          })());

const nodeUrl = process.argv[3] || DEFAULT_NODE_URL;

async function main() {
    console.log(`Contract: ${contractAddress}\nNode: ${nodeUrl}\n`);

    const client = createClient(getWsProvider(nodeUrl));
    const registry = createInkSdk(client).getContract(
        contracts.nameRegistry,
        contractAddress,
    );
    const signer = prepareSigner("Alice");

    // Query before register
    const before = await registry.query("myName", { origin: ALICE, data: {} });
    console.log(
        `myName (before): "${before.success ? before.value.response : "FAILED"}"`,
    );

    // Register a name
    console.log(`\nRegistering "Alice"...`);
    const tx = await registry
        .send("register", {
            data: { name: "Alice" },
            gasLimit: GAS_LIMIT,
            storageDepositLimit: STORAGE_DEPOSIT_LIMIT,
        })
        .signAndSubmit(signer);
    console.log(`Tx: ${tx.block.hash}`);

    // Query after register
    const after = await registry.query("myName", { origin: ALICE, data: {} });
    console.log(
        `myName (after): "${after.success ? after.value.response : "FAILED"}"`,
    );

    // Query total registrations
    const total = await registry.query("totalRegistrations", {
        origin: ALICE,
        data: {},
    });
    console.log(
        `totalRegistrations: ${total.success ? total.value.response : "FAILED"}`,
    );

    client.destroy();
}

main().catch((e) => {
    console.error(e);
    process.exit(1);
});
