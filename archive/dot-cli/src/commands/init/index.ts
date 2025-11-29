import { Command } from "commander";
import { execSync } from "child_process";

export const c = "";

/**
 * ## Init
 */
export function initHardHat(...args: string[]) {
    const dir = args[0];
    execSync(
        `
        mkdir -p ${dir}/bin
        cd ${dir}

        npm init -y
        npm install --save-dev @parity/hardhat-polkadot@latest
        npx hardhat-polkadot init -y

        # Detect system architecture
        ARCH=$(uname -m)
        if [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
            ARCH_SUFFIX="-arm64"
        else
            ARCH_SUFFIX="-linux-x64"
        fi

        # Download dev-node binary
        wget -q --show-progress -O ./bin/dev-node \
            https://github.com/paritytech/hardhat-polkadot/releases/download/nodes-latest/revive-dev-node$ARCH_SUFFIX

        # Download eth-rpc binary
        wget -q --show-progress -O ./bin/eth-rpc \
            https://github.com/paritytech/hardhat-polkadot/releases/download/nodes-latest/eth-rpc$ARCH_SUFFIX
        chmod +x ./bin/*
        `,
        { stdio: "inherit" }
    );
}

export function initFoundry(...args: string[]) {
    const dir = args[0];
    console.log(`HERE IT IS [foundry]`, dir);
}

export function initInk(...args: string[]) {
    const dir = args[0];
    console.log(`HERE IT IS [ink]`, dir);
}
