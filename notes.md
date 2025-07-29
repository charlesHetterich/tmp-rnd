Mindful of
- easy CI
- time to setup, first time
- time to setup, each time
- space taken on device
- development from local or remote machine. Do we have to be mindful of windows ?
- target users
    - very basic developers
    - high level developers at companies w/ potentially weird permissions and things such as GPG key forwarding for verified commits 🤔
- Can we forward a users development setup? Balance between using a user's custom configurations (such as using Oh My Zsh), vs. things we want to *override* within the dev container (i.e. formatters)

Requires
- Docker
```
curl -fsSL https://get.docker.com | sudo sh

# mostly for CI
sudo groupadd docker        # no‑op if the group already exists
sudo usermod -aG docker $USER
newgrp docker                # re‑eval groups without logging out
```
- VSCode Dev Containers plugin