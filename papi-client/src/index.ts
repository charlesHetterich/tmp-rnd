import { createClient } from "polkadot-api";
import { getWsProvider, WsEvent } from "polkadot-api/ws-provider";
import { withPolkadotSdkCompat } from "polkadot-api/polkadot-sdk-compat";

// Import generated descriptors (created by `npm run codegen`)
import { relay } from "@polkadot-api/descriptors";
import { asset_hub } from "@polkadot-api/descriptors";

// Well-known development addresses
const ALICE = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

// Initial delay to let zombienet chains stabilize (in seconds)
const INITIAL_DELAY = parseInt(process.env.DELAY || "5", 10);

async function main() {
  console.log("=".repeat(60));
  console.log("PAPI Client - Connecting to Local Chains");
  console.log("=".repeat(60));
  console.log();

  if (INITIAL_DELAY > 0) {
    console.log(`Waiting ${INITIAL_DELAY}s for chains to stabilize...`);
    await new Promise((r) => setTimeout(r, INITIAL_DELAY * 1000));
  }

  console.log("Connecting...");

  // Track connection state (only log meaningful events)
  let relayConnectedOnce = false;
  let assetHubConnectedOnce = false;

  // Create WebSocket providers with minimal logging
  const relayProvider = getWsProvider("ws://127.0.0.1:9944", (status) => {
    if (status.type === WsEvent.CONNECTED && !relayConnectedOnce) {
      relayConnectedOnce = true;
      console.log("  [Relay] WebSocket connected");
    }
  });

  const assetHubProvider = getWsProvider("ws://127.0.0.1:9946", (status) => {
    if (status.type === WsEvent.CONNECTED && !assetHubConnectedOnce) {
      assetHubConnectedOnce = true;
      console.log("  [Asset Hub] WebSocket connected");
    }
  });

  // Create clients with polkadot-sdk-compat for local testnets
  const relayClient = createClient(withPolkadotSdkCompat(relayProvider));
  const assetHubClient = createClient(withPolkadotSdkCompat(assetHubProvider));

  // Get typed APIs
  const relayApi = relayClient.getTypedApi(relay);
  const assetHubApi = assetHubClient.getTypedApi(asset_hub);

  // Use best block instead of finalized for faster connection
  let relayConnected = false;
  let assetHubConnected = false;
  let relayBlock: { number: number; hash: string } | null = null;
  let assetHubBlock: { number: number; hash: string } | null = null;

  // Subscribe to best blocks to detect when connected
  const relaySub = relayClient.bestBlocks$.subscribe((blocks) => {
    if (!relayConnected && blocks.length > 0) {
      relayConnected = true;
      relayBlock = { number: blocks[0].number, hash: blocks[0].hash };
      console.log(`  [Relay] Ready at block #${blocks[0].number}`);
    }
  });

  const assetHubSub = assetHubClient.bestBlocks$.subscribe((blocks) => {
    if (!assetHubConnected && blocks.length > 0) {
      assetHubConnected = true;
      assetHubBlock = { number: blocks[0].number, hash: blocks[0].hash };
      console.log(`  [Asset Hub] Ready at block #${blocks[0].number}`);
    }
  });

  // Wait for both to connect (with longer timeout for slow startups)
  const timeout = 120000; // 2 minutes
  const start = Date.now();
  while ((!relayConnected || !assetHubConnected) && Date.now() - start < timeout) {
    await new Promise((r) => setTimeout(r, 1000));
    // Show progress dots every 10 seconds
    if ((Date.now() - start) % 10000 < 1000) {
      const elapsed = Math.floor((Date.now() - start) / 1000);
      const missing = [];
      if (!relayConnected) missing.push("relay");
      if (!assetHubConnected) missing.push("asset-hub");
      console.log(`  Waiting for: ${missing.join(", ")} (${elapsed}s)`);
    }
  }

  if (!relayConnected || !assetHubConnected) {
    console.error("\nFailed to connect to chains within timeout");
    console.error("  Relay connected:", relayConnected);
    console.error("  Asset Hub connected:", assetHubConnected);
    console.error("\nMake sure zombienet is running: cd .. && ./run.sh");
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
  console.log(`  Block: #${relayBlock!.number}`);
  console.log(`  Hash:  ${relayBlock!.hash}`);

  // Get relay chain runtime version
  const relayVersion = await relayApi.constants.System.Version();
  console.log(`  Spec:  ${relayVersion.spec_name} v${relayVersion.spec_version}`);

  // Check Alice's balance on relay chain
  const aliceAccount = await relayApi.query.System.Account.getValue(ALICE);
  const freeBalance = aliceAccount.data.free;
  // Relay chain uses 12 decimals (1 ROC = 10^12 planck)
  const balanceInROC = Number(freeBalance) / 1e12;
  console.log(`  Alice: ${balanceInROC.toFixed(4)} ROC`);

  console.log();
  console.log("-".repeat(60));
  console.log("ASSET HUB (Parachain 1000)");
  console.log("-".repeat(60));
  console.log(`  Block: #${assetHubBlock!.number}`);
  console.log(`  Hash:  ${assetHubBlock!.hash}`);

  // Get Asset Hub runtime version
  const assetHubVersion = await assetHubApi.constants.System.Version();
  console.log(`  Spec:  ${assetHubVersion.spec_name} v${assetHubVersion.spec_version}`);

  // Check Alice's balance on Asset Hub
  const aliceAssetHubAccount = await assetHubApi.query.System.Account.getValue(ALICE);
  const assetHubFreeBalance = aliceAssetHubAccount.data.free;
  const assetHubBalanceInROC = Number(assetHubFreeBalance) / 1e12;
  console.log(`  Alice: ${assetHubBalanceInROC.toFixed(4)} ROC`);

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
