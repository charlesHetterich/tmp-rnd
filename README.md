# Smart Contract Dev Containers
This repository is for R&D purposes, currently focused on developing [Dev Containers](https://containers.dev/) for Smart Contract development on Polkadot.

### Quick Started
1. In your projects root directory, fetch our dev container environment:
```
curl -L --create-dirs \
     -o .devcontainer/devcontainer.json \
     https://raw.githubusercontent.com/charlesHetterich/scc-rnd/main/.devcontainer/devcontainer.json
```
2. VScode should prompt you to "Reopen in container". Do this—Make sure the "Dev Containers" plugin is installed

3. After the container loads, follow the startup instructions to load the generated Paseo address with faucet tokens.