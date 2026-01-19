import { createClient } from "polkadot-api";
import { getWsProvider, WsEvent } from "polkadot-api/ws-provider";
import { withPolkadotSdkCompat } from "polkadot-api/polkadot-sdk-compat";

// Import generated descriptors (created by `npm run codegen`)
import { relay } from "@polkadot-api/descriptors";
import { asset_hub } from "@polkadot-api/descriptors";

// Well-known development addresses
const ALICE = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

async function main() {
  console.log("=".repeat(60));
  console.log("PAPI Client - Connecting to Local Chains");
  console.log("=".repeat(60));
  console.log();

  // Create WebSocket providers with status logging
  const relayProvider = getWsProvider("ws://127.0.0.1:9944", (status) => {
    const eventName = WsEvent[status.type];
    console.log(`  [Relay WS] ${eventName}`);
  });

  const assetHubProvider = getWsProvider("ws://127.0.0.1:9946", (status) => {
    const eventName = WsEvent[status.type];
    console.log(`  [Asset Hub WS] ${eventName}`);
  });

  // Create clients with polkadot-sdk-compat for local testnets
  const relayClient = createClient(withPolkadotSdkCompat(relayProvider));
  const assetHubClient = createClient(withPolkadotSdkCompat(assetHubProvider));

  // Get typed APIs
  const relayApi = relayClient.getTypedApi(relay);
  const assetHubApi = assetHubClient.getTypedApi(asset_hub);

  console.log("Connecting to chains...");

  // Use best block instead of finalized for faster connection
  let relayConnected = false;
  let assetHubConnected = false;

  // Subscribe to best blocks to detect when connected
  const relaySub = relayClient.bestBlocks$.subscribe((blocks) => {
    if (!relayConnected && blocks.length > 0) {
      relayConnected = true;
      console.log(`  Relay chain connected at block #${blocks[0].number}`);
    }
  });

  const assetHubSub = assetHubClient.bestBlocks$.subscribe((blocks) => {
    if (!assetHubConnected && blocks.length > 0) {
      assetHubConnected = true;
      console.log(`  Asset Hub connected at block #${blocks[0].number}`);
    }
  });

  // Wait for both to connect (with timeout)
  const timeout = 30000;
  const start = Date.now();
  while ((!relayConnected || !assetHubConnected) && Date.now() - start < timeout) {
    await new Promise((r) => setTimeout(r, 500));
  }

  if (!relayConnected || !assetHubConnected) {
    console.error("Failed to connect to chains within timeout");
    relaySub.unsubscribe();
    assetHubSub.unsubscribe();
    relayClient.destroy();
    assetHubClient.destroy();
    process.exit(1);
  }

  console.log();
  console.log("-".repeat(60));
  console.log("RELAY CHAIN");
  console.log("-".repeat(60));

  // Get relay chain runtime version
  const relayVersion = await relayApi.constants.System.Version();
  console.log(`  Spec: ${relayVersion.spec_name} v${relayVersion.spec_version}`);

  // Check Alice's balance on relay chain
  const aliceAccount = await relayApi.query.System.Account.getValue(ALICE);
  const freeBalance = aliceAccount.data.free;
  // Relay chain uses 12 decimals (1 ROC = 10^12 planck)
  const balanceInROC = Number(freeBalance) / 1e12;
  console.log(`  Alice balance: ${balanceInROC.toFixed(4)} ROC`);

  console.log();
  console.log("-".repeat(60));
  console.log("ASSET HUB");
  console.log("-".repeat(60));

  // Get Asset Hub runtime version
  const assetHubVersion = await assetHubApi.constants.System.Version();
  console.log(`  Spec: ${assetHubVersion.spec_name} v${assetHubVersion.spec_version}`);

  // Check Alice's balance on Asset Hub
  const aliceAssetHubAccount = await assetHubApi.query.System.Account.getValue(ALICE);
  const assetHubFreeBalance = aliceAssetHubAccount.data.free;
  const assetHubBalanceInROC = Number(assetHubFreeBalance) / 1e12;
  console.log(`  Alice balance: ${assetHubBalanceInROC.toFixed(4)} ROC`);

  console.log();
  console.log("-".repeat(60));
  console.log("SUBSCRIBING TO NEW BLOCKS (10 seconds)");
  console.log("-".repeat(60));

  // Subscribe to best blocks on both chains
  const relayBlockSub = relayClient.bestBlocks$.subscribe((blocks) => {
    if (blocks.length > 0) {
      console.log(`  [Relay]     Best block #${blocks[0].number}`);
    }
  });

  const assetHubBlockSub = assetHubClient.bestBlocks$.subscribe((blocks) => {
    if (blocks.length > 0) {
      console.log(`  [Asset Hub] Best block #${blocks[0].number}`);
    }
  });

  // Wait 10 seconds to observe blocks
  await new Promise((resolve) => setTimeout(resolve, 10000));

  // Cleanup
  console.log();
  console.log("Disconnecting...");
  relaySub.unsubscribe();
  assetHubSub.unsubscribe();
  relayBlockSub.unsubscribe();
  assetHubBlockSub.unsubscribe();
  relayClient.destroy();
  assetHubClient.destroy();

  console.log("Done!");
  console.log();
}

main().catch((err) => {
  console.error("Error:", err);
  process.exit(1);
});
