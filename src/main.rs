use std::time::Duration;

use anyhow::{Error, anyhow};
use serde_json::json;
use zombienet_sdk::{
    LocalFileSystem, Network, NetworkConfig, NetworkConfigBuilder, NetworkConfigExt,
    subxt::ext::futures::future::try_join,
};

mod config;
use config::ChainConfig;

fn generate_config_main_network(
    chain_config: &ChainConfig,
) -> Result<NetworkConfig, anyhow::Error> {
    let config = NetworkConfigBuilder::new()
        .with_relaychain(|r| {
            r.with_chain("westend-local")
                .with_default_command(chain_config.relay_chain_command.as_str())
                .with_chain_spec_runtime(
                    chain_config.relay_chain_runtime_path.as_str(),
                    Some("local"),
                )
                .with_genesis_overrides(json!({
                    "configuration": {
                        "config": {
                        "node_features": {
                              "bits": 8,
                              "data": [
                                11
                              ],
                              "head": {
                                "index": 0,
                                "width": 8
                              },
                              "order": "bitvec::order::Lsb0"
                            },
                        }
                    }
                }))
                .with_validator(|n| n.with_name("validator-1"))
                .with_validator(|n| n.with_name("validator-2"))
                .with_validator(|n| n.with_name("validator-3"))
                .with_validator(|n| n.with_name("validator-4"))
                .with_validator(|n| n.with_name("validator-5"))
        })
        // Asset Hub with revive support
        .with_parachain(|p| {
            p.with_id(1000)
                .with_chain("asset-hub-polkadot-local")
                .with_chain_spec_runtime(
                    chain_config.asset_hub_runtime_path.as_str(),
                    Some("local"),
                )
                .with_default_command(chain_config.parachain_command.as_str())
                .with_collator(|c| {
                    c.with_name("collator-ah").with_args(vec![
                        "-lbasic-authorship=trace".into(),
                        "-lruntime::revive=debug".into(),
                    ])
                })
        })
        // People chain with PoP & statement store
        .with_parachain(|p| {
            p.with_id(1004)
                .with_chain("people-polkadot-local")
                .with_chain_spec_runtime(chain_config.people_runtime_path.as_str(), Some("local"))
                .with_default_command(chain_config.parachain_command.as_str())
                .with_collator(|c| {
                    c.with_name("collator-people").with_args(vec![
                        "-lbasic-authorship=trace".into(),
                        "-lruntime::pop=debug".into(),
                        "-lruntime::statement=debug".into(),
                    ])
                })
        })
        // YAP chain
        .with_parachain(|p| {
            p.with_id(2000)
                .with_chain("yap-polkadot-local-2000")
                .with_chain_spec_runtime(
                    chain_config.yap_runtime_path.as_str(),
                    Some("local_testnet"),
                )
                .with_default_command(chain_config.parachain_command.as_str())
                .with_collator(|c| {
                    c.with_name("collator-yap")
                        .with_args(vec!["-lbasic-authorship=trace".into()])
                })
        })
        // Template chain
        .with_parachain(|p| {
            p.with_id(2001)
                .with_chain("para-template")
                .with_chain_spec_runtime(
                    chain_config.template_runtime_path.as_str(),
                    Some("local_testnet"),
                )
                .with_default_command(chain_config.parachain_command.as_str())
                .with_collator(|c| {
                    c.with_name("collator-tmpl")
                        .with_args(vec!["-lbasic-authorship=trace".into()])
                })
        })
        .build()
        .map_err(|e| {
            let errs = e
                .into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            anyhow!("config errs: {errs}")
        })?;

    Ok(config)
}

fn generate_config_bulletin(chain_config: &ChainConfig) -> Result<NetworkConfig, anyhow::Error> {
    let config = NetworkConfigBuilder::new()
        .with_relaychain(|r| {
            r.with_chain("bulletin-polkadot-local")
                .with_default_command(chain_config.bulletin_command.as_str())
                .with_validator(|n| n.with_name("alice"))
        })
        .build()
        .map_err(|e| {
            let errs = e
                .into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            anyhow!("config errs: {errs}")
        })?;

    Ok(config)
}

fn print_nodes_info(network: &Network<LocalFileSystem>) {
    for node in network.nodes() {
        println!(
            "💻 {}: direct link (pjs) https://polkadot.js.org/apps/?rpc={}#/explorer",
            node.name(),
            node.ws_uri()
        );
        println!(
            "💻 {}: direct link (papi) https://dev.papi.how/explorer#networkId=custom&endpoint={}",
            node.name(),
            node.ws_uri()
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let chain_config = ChainConfig::default();

    let main_config = generate_config_main_network(&chain_config)?;
    let bulletin_config = generate_config_bulletin(&chain_config)?;
    let (main_network, bulletin_network) =
        try_join(main_config.spawn_native(), bulletin_config.spawn_native()).await?;

    println!("🚀🚀🚀🚀 networks deployed");

    println!("\n=== Main network info ===");
    print_nodes_info(&main_network);

    println!("\n=== Bulletin network info ===");
    print_nodes_info(&bulletin_network);

    // For now let just loop....
    #[allow(clippy::empty_loop)]
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
