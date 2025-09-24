test_port() {
    pnpm dlx tsx /home/hardhat-polkadot/packages/hardhat-polkadot/src/cli port "$1" "$2"
}

test_port "./bin/interchain-token-service" "$1"
test_port "./bin/redstone-oracles-monorepo/packages/eth-contracts" "$1"
test_port "./bin/fireblocks-smart-contracts" "$1"
test_port "./bin/chainlink-evm/contracts" "$1"
test_port "./bin/ccip/contracts" "$1"
