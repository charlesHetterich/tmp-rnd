# Smart Contract Dev Containers

1. In your projects repository, fetch our dev container environment with the following command.
```
curl -L --create-dirs \
     -o .devcontainer/devcontainer.json \
     https://raw.githubusercontent.com/charlesHetterich/scc-rnd/main/.devcontainer/devcontainer.json
```

2. Re-open inside the Dev Container (VSCode should prompt you for this. Make sure the "Dev Containers" plugin is installed)

3. When the container loads, follow the startup instructions to load the generated Paseo address with faucet tokens.