# Clean existing repos
for dir in bin/*/; do
    if [ -d "${dir}/.git" ]; then
        echo "Removing git repository: $dir"
        rm -rf "$dir"
    fi
done

mkdir -p bin
git -C ./bin clone https://github.com/axelarnetwork/interchain-token-service.git      # interchain-token-service [Axelar]
git -C ./bin clone https://github.com/redstone-finance/redstone-oracles-monorepo.git  # redstone-oracles-monorepo
git -C ./bin clone https://github.com/fireblocks/fireblocks-smart-contracts.git       # fireblocks-smart-contracts
git -C ./bin clone https://github.com/smartcontractkit/chainlink-evm.git              # chainlink-evm
git -C ./bin clone https://github.com/aave/ccip.git                                   # Aave CCIP
