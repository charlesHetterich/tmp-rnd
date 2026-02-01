/**
 * Query and interact with the name-registry contract using typed papi descriptors
 *
 * Uses getSolidityClient for typed encode/decode (similar to getInkClient for ink!)
 */
import { createClient } from "polkadot-api";
import { getWsProvider } from "polkadot-api/ws-provider/web";
import { getPolkadotSigner } from "polkadot-api/signer";
import { assetHub, contracts } from "@polkadot-api/descriptors";
import { sr25519CreateDerive } from "@polkadot-labs/hdkd";
import { createInkSdk } from "@polkadot-api/sdk-ink";
import {
    DEV_PHRASE,
    entropyToMiniSecret,
    mnemonicToEntropy,
} from "@polkadot-labs/hdkd-helpers";
import { SS58String } from "polkadot-api";
import { ContractDeployer } from "./deployer";

// Contract address from deployment
const CONTRACT_ADDRESS = process.argv[2] || "todo!!";

const deployer = new ContractDeployer("Alice");
deployer;

// Alice's SS58 address
const ALICE_SS58 =
    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" as SS58String;

// Alice's dev account
const miniSecret = entropyToMiniSecret(mnemonicToEntropy(DEV_PHRASE));
const derive = sr25519CreateDerive(miniSecret);
const aliceKeyPair = derive("//Alice");
const alice = getPolkadotSigner(
    aliceKeyPair.publicKey,
    "Sr25519",
    aliceKeyPair.sign,
);

// Connect to Asset Hub
console.log("\nConnecting to Asset Hub...");
const client = createClient(getWsProvider("ws://127.0.0.1:10020"));
const api = client.getTypedApi(assetHub);

const inkSdk = createInkSdk(client);
const nameRegistryContract = inkSdk.getContract(
    contracts.nameRegistry,
    CONTRACT_ADDRESS,
);

const result = nameRegistryContract.query("register", {
    origin: ALICE_SS58,
    data: { name: 232n },
});
