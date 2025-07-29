require("@nomicfoundation/hardhat-toolbox");
require("@parity/hardhat-polkadot");

/**
 * @type import('hardhat/config').HardhatUserConfig
 */
module.exports = {
  solidity: {
    version: "0.8.23",
  },
  networks: {
    hardhat: {
      polkavm: true,
      nodeConfig: {
        nodeBinaryPath:
          "/home/vscode/polkadot-sdk/target/release/substrate-node",
        rpcPort: 8000,
        dev: true,
      },
      adapterConfig: {
        adapterBinaryPath: "/home/vscode/polkadot-sdk/target/release/eth-rpc",
        dev: true,
      },
    },
  },
  resolc: {
    compilerSource: "binary",
    settings: {
      compilerPath: "/home/vscode/resolc-x86_64-unknown-linux-musl",
      optimizer: {
        enabled: true,
        parameters: "z",
        fallbackOz: true,
        runs: 200,
      },
      standardJson: true,
    },
  },
};
