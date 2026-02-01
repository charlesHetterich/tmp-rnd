# Get dependencies for zombienet
curl -sSL https://raw.githubusercontent.com/paritytech/ppn-proxy/main/install.sh | bash

# Get reference repositories for Claude
rm -rf ./references
mkdir -p ./references
git clone https://github.com/use-ink/ink ./references/ink
git clone https://github.com/polkadot-api/polkadot-api.git ./references/polkadot-api
git clone https://github.com/paritytech/cargo-pvm-contract.git ./references/cargo-pvm-contract --depth 1 --branch feat/abi-generation-and-macro-improvements