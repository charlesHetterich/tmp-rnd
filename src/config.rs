#![allow(dead_code)]
/// Configuration for chain versions and runtime paths

#[derive(Debug, Clone)]
pub struct ChainConfig {
    pub relay_chain_command: String,
    pub relay_chain_runtime_path: String,
    pub parachain_command: String,
    pub bulletin_command: String,
    pub bridge_hub_command: String,

    // Runtime paths for each chain
    pub asset_hub_runtime_path: String,
    pub people_runtime_path: String,
    pub yap_runtime_path: String,
    pub template_runtime_path: String,
    pub bridge_hub_runtime_path: String,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            relay_chain_command: "polkadot".to_string(),
            relay_chain_runtime_path:
                "./runtime/polkadot_runtime-v2000004.compact.compressed.wasm ".to_string(),
            parachain_command: "polkadot-parachain".to_string(),
            bulletin_command: "polkadot-bulletin-chain".to_string(),
            bridge_hub_command: "polkadot-parachain".to_string(),

            asset_hub_runtime_path:
                "./runtime/asset-hub-polkadot_runtime-v2000004.compact.compressed.wasm ".to_string(),
            people_runtime_path: "./runtime/people_rococo_runtime.compact.compressed.wasm"
                .to_string(),
            template_runtime_path: "./runtime/parachain_template_runtime.compact.compressed"
                .to_string(),

            // TODO
            bridge_hub_runtime_path: "/tmp/bridge_hub_polkadot_runtime.compact.compressed.wasm"
                .to_string(),
            yap_runtime_path: "/tmp/yet_another_parachain_runtime.compact.compressed.wasm"
                .to_string(),
        }
    }
}

impl ChainConfig {
    /// Create a new ChainConfig with custom paths
    pub fn new() -> Self {
        Self::default()
    }

    /// Set custom relay chain command
    pub fn with_relay_chain_command(mut self, command: impl Into<String>) -> Self {
        self.relay_chain_command = command.into();
        self
    }

    /// Set custom relay chain runtime path
    pub fn with_relay_chain_runtime(mut self, path: impl Into<String>) -> Self {
        self.relay_chain_runtime_path = path.into();
        self
    }

    /// Set custom parachain command
    pub fn with_parachain_command(mut self, command: impl Into<String>) -> Self {
        self.parachain_command = command.into();
        self
    }

    /// Set custom Asset Hub runtime path
    pub fn with_asset_hub_runtime(mut self, path: impl Into<String>) -> Self {
        self.asset_hub_runtime_path = path.into();
        self
    }

    /// Set custom People chain runtime path
    pub fn with_people_runtime(mut self, path: impl Into<String>) -> Self {
        self.people_runtime_path = path.into();
        self
    }

    /// Set custom BridgeHub runtime path
    pub fn with_bridge_hub_runtime(mut self, path: impl Into<String>) -> Self {
        self.bridge_hub_runtime_path = path.into();
        self
    }

    /// Set custom YAP runtime path
    pub fn with_yap_runtime(mut self, path: impl Into<String>) -> Self {
        self.yap_runtime_path = path.into();
        self
    }

    /// Set custom template runtime path
    pub fn with_template_runtime(mut self, path: impl Into<String>) -> Self {
        self.template_runtime_path = path.into();
        self
    }

    /// Set custom bulletin command
    pub fn with_bulletin_command(mut self, command: impl Into<String>) -> Self {
        self.bulletin_command = command.into();
        self
    }
}
