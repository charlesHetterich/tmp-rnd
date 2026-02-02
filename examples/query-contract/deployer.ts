import { Binary, FixedSizeBinary, PolkadotClient } from "polkadot-api";
import { assetHub } from "@polkadot-api/descriptors";
import { prepareSigner } from "./signer";
import { GAS_LIMIT, STORAGE_DEPOSIT_LIMIT } from "./constants";

/**
 * Deploy a PolkaVM contract and return its address.
 */
export async function deployContract(
  client: PolkadotClient,
  bytecode: Uint8Array,
  signer = prepareSigner("Alice"),
): Promise<string> {
  const api = client.getTypedApi(assetHub);

  const result = await api.tx.Revive.instantiate_with_code({
    value: 0n,
    weight_limit: GAS_LIMIT,
    storage_deposit_limit: STORAGE_DEPOSIT_LIMIT,
    code: Binary.fromBytes(bytecode),
    data: Binary.fromBytes(new Uint8Array(0)),
    salt: undefined,
  }).signAndSubmit(signer);

  const event = result.events.find(e => e.type === "Revive" && e.value.type === "Instantiated");
  if (!event) throw new Error("Deployment failed - no Instantiated event");

  return (event.value.value as { contract: FixedSizeBinary<20> }).contract.asHex();
}
