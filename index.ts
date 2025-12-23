// // import { createInkSdk } from "@polkadot-api/sdk-ink";
// import { createClient } from "polkadot-api";
// import { withPolkadotSdkCompat } from "polkadot-api/polkadot-sdk-compat";
// import { getWsProvider } from "polkadot-api/ws-provider";
// import * as D from "@polkadot-api/descriptors";
// import { FixedSizeBinary } from "polkadot-api";

// // ink.getInkClient

// const client = createClient(
//     withPolkadotSdkCompat(getWsProvider("wss://testnet-passet-hub.polkadot.io"))
// );

// const api = client.getTypedApi(D.passet);

// const contractInfo = await api.query.Revive.AccountInfoOf.getValue(
//     FixedSizeBinary.fromHex("0x0b6670b0185b23df080b340fac8948fa2b0e7c62")
// );
// const codeHash = contractInfo?.account_type.value?.code_hash;

// console.log(codeHash?.asHex());

// const codeInfo = await api.query.Revive.PristineCode.getValue(codeHash!);

// console.log(codeInfo?.asText());

import { writeFileSync } from "node:fs";
import { createClient } from "polkadot-api";
import { withPolkadotSdkCompat } from "polkadot-api/polkadot-sdk-compat";
import { getWsProvider } from "polkadot-api/ws-provider";
import * as D from "@polkadot-api/descriptors";
import { FixedSizeBinary } from "polkadot-api";

const client = createClient(
    withPolkadotSdkCompat(getWsProvider("wss://testnet-passet-hub.polkadot.io"))
);

const api = client.getTypedApi(D.passet);

const addr = FixedSizeBinary.fromHex(
    "0x0b6670b0185b23df080b340fac8948fa2b0e7c62"
);

const contractInfo = await api.query.Revive.AccountInfoOf.getValue(addr);
const codeHash = contractInfo?.account_type.value?.code_hash;
if (!codeHash) throw new Error("no code_hash (is this address a contract?)");

const pristine = await api.query.Revive.PristineCode.getValue(codeHash);
if (!pristine) throw new Error("no PristineCode for that code_hash");

const bytes = pristine.asBytes(); // Uint8Array
writeFileSync("contract.polkavm", Buffer.from(bytes));

console.log({
    codeHash: codeHash.asHex(),
    byteLen: bytes.length,
    headHex: Buffer.from(bytes).subarray(0, 32).toString("hex"),
});
