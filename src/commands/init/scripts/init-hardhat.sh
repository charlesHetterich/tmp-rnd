TARGET_DIR=$1

npm init -y
npx hardhat-polkadot init -y
mkdir -p $TARGET_DIR/bin/
npm install --save-dev @parity/hardhat-polkadot@latest
npx hardhat-polkadot init -y
chmod +x $TARGET_DIR/bin/*

# Download dev-node binary
wget -q --show-progress -O $TARGET_DIR/bin/dev-node \
    https://github.com/paritytech/hardhat-polkadot/releases/download/nodes-latest/revive-dev-node-linux-x64

# Download eth-rpc binary
wget -q --show-progress -O $TARGET_DIR/bin/eth-rpc \
    https://github.com/paritytech/hardhat-polkadot/releases/download/nodes-latest/eth-rpc-linux-x64