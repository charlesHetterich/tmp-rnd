import {
    PolkadotClient,
    TypedApi,
    Binary,
    FixedSizeBinary,
} from "polkadot-api";
import { AssetHub, contracts } from "@polkadot-api/descriptors";
import { createInkSdk } from "@polkadot-api/sdk-ink";
import { readFileSync, existsSync } from "fs";
import { resolve } from "path";
import { execSync } from "child_process";
import { prepareSigner } from "./signer";
import { GAS_LIMIT, STORAGE_DEPOSIT_LIMIT } from "./constants";

export class ContractDeployer {
    public signer: ReturnType<typeof prepareSigner>;
    public api!: TypedApi<AssetHub>;
    public client!: PolkadotClient;
    public lastDeployedAddr: string | null = null;

    constructor(signerName: string = "Alice") {
        this.signer = prepareSigner(signerName);
    }

    /**
     * Set the API connection.
     */
    setConnection(client: PolkadotClient, api: TypedApi<AssetHub>) {
        this.client = client;
        this.api = api;
    }

    /**
     * Deploy a contract and return its address.
     * @param contractPath - Path to the .contract file
     * @returns The deployed contract's address (H160)
     */
    async deploy(contractPath: string): Promise<string> {
        const contract = JSON.parse(readFileSync(contractPath, "utf-8"));
        const code = Binary.fromHex(contract.source.contract_binary);
        const constructor = contract.spec.constructors.find(
            (c: { label: string }) => c.label === "new",
        );
        if (!constructor) {
            throw new Error(`No "new" constructor found in ${contractPath}`);
        }
        const data = Binary.fromHex(constructor.selector);

        const result = await this.api.tx.Revive.instantiate_with_code({
            value: 0n,
            weight_limit: {
                ref_time: GAS_LIMIT.refTime,
                proof_size: GAS_LIMIT.proofSize,
            },
            storage_deposit_limit: STORAGE_DEPOSIT_LIMIT,
            code,
            data,
            salt: undefined,
        }).signAndSubmit(this.signer);

        const instantiatedEvent = result.events.find(
            (e) => e.type === "Revive" && e.value.type === "Instantiated",
        );
        if (!instantiatedEvent) {
            throw new Error(
                "Contract instantiation failed - no Instantiated event",
            );
        }
        const contractAddr = (
            instantiatedEvent.value.value as { contract: FixedSizeBinary<20> }
        ).contract;
        this.lastDeployedAddr = contractAddr.asHex();
        return this.lastDeployedAddr;
    }
}
