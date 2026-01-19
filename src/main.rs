use std::time::Duration;
use anyhow::{Error, anyhow};
use zombienet_sdk::{
    LocalFileSystem, Network, NetworkConfig, NetworkConfigBuilder, NetworkConfigExt,
};

mod config;
use config::ChainConfig;

/// Relay chain (Rococo local) with Asset Hub parachain
fn relay_with_asset_hub(cfg: &ChainConfig) -> Result<NetworkConfig, anyhow::Error> {
    NetworkConfigBuilder::new()
        .with_relaychain(|r| {
            r.with_chain("rococo-local")
                .with_default_command(cfg.relay_chain_command.as_str())
                .with_validator(|n| n.with_name("alice"))
                .with_validator(|n| n.with_name("bob"))
        })
        .with_parachain(|p| {
            p.with_id(1000)
                .with_chain("asset-hub-rococo-local")
                .with_default_command(cfg.parachain_command.as_str())
                .with_collator(|c| c.with_name("asset-hub-collator"))
        })
        .build()
        .map_err(|e| anyhow!("relay config errs: {}", e.into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join(" ")))
}

/// Bulletin standalone chain (solochain, not a parachain)
fn bulletin_standalone(cfg: &ChainConfig) -> Result<NetworkConfig, anyhow::Error> {
    NetworkConfigBuilder::new()
        .with_relaychain(|r| {
            r.with_chain("local")
                .with_default_command(cfg.bulletin_command.as_str())
                .with_validator(|n| n.with_name("bulletin-alice"))
                .with_validator(|n| n.with_name("bulletin-bob"))
        })
        .build()
        .map_err(|e| anyhow!("bulletin config errs: {}", e.into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join(" ")))
}

fn print_nodes_info(label: &str, network: &Network<LocalFileSystem>) {
    println!("\n{}:", label);
    for node in network.nodes() {
        println!("  {}: https://polkadot.js.org/apps/?rpc={}#/explorer", node.name(), node.ws_uri());
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let cfg = ChainConfig::default();

    println!("Starting relay chain + Asset Hub...");
    let relay_net = relay_with_asset_hub(&cfg)?.spawn_native().await?;

    // Small delay to let relay chain start producing blocks
    tokio::time::sleep(Duration::from_secs(5)).await;

    println!("Starting bulletin chain (standalone)...");
    let bulletin_net = bulletin_standalone(&cfg)?.spawn_native().await?;

    println!("\n=== All networks deployed ===");
    print_nodes_info("Relay Chain + Asset Hub", &relay_net);
    print_nodes_info("Bulletin Chain (standalone)", &bulletin_net);

    println!("\n---");
    println!("Relay chain: alice, bob (validators)");
    println!("Asset Hub: asset-hub-collator (parachain 1000)");
    println!("Bulletin: bulletin-alice, bulletin-bob (standalone chain validators)");
    println!("\nConnect to each node in Polkadot.js to verify blocks are being produced.");

    loop { tokio::time::sleep(Duration::from_secs(60)).await; }
}
