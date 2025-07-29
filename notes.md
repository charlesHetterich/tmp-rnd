Mindful of
- easy CI
- time to setup, first time
- time to setup, each time
- space taken on device
- development from local or remote machine. Do we have to be mindful of windows ?

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