/**
 * Deploy name-registry contract
 * Usage: pnpm deploy [node-url]
 */
import { createClient } from "polkadot-api";
import { getWsProvider } from "polkadot-api/ws-provider/web";
import { readFileSync, writeFileSync } from "fs";
import { resolve } from "path";
import { deployContract } from "./deployer";
import { DEFAULT_NODE_URL } from "./constants";

const nodeUrl = process.argv[2] || DEFAULT_NODE_URL;
const bytecode = readFileSync(resolve(import.meta.dirname, "../../target/name-registry.release.polkavm"));

async function main() {
  console.log(`Deploying to ${nodeUrl}...`);

  const client = createClient(getWsProvider(nodeUrl));
  const addr = await deployContract(client, bytecode);

  console.log(`Deployed: ${addr}`);
  writeFileSync(resolve(import.meta.dirname, "contract-address.txt"), addr);

  client.destroy();
}

main().catch(e => { console.error(e); process.exit(1); });
