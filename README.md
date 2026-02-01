```
bash `scripts/setup.sh`
```

Start the zombienet:

```
BIN=$(pwd)/bin zombie-cli spawn -p native ./bin/local-dev.toml
```

build the contract:
Generates `.polkavm` and `.abi.json` files.

```
cargo build --release -p name-registry
```
