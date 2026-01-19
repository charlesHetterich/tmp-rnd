#[derive(Debug, Clone)]
pub struct ChainConfig {
    pub relay_chain_command: String,
    pub parachain_command: String,
    pub bulletin_command: String,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            relay_chain_command: "polkadot".into(),
            parachain_command: "polkadot-parachain".into(),
            bulletin_command: "polkadot-bulletin-chain".into(),
        }
    }
}
