# Generate testing address if none exists yet
if [ ! -s ~/.address.json ]; then
    subkey generate --network polkadot --output-type json > ~/.address.json
fi
addr=$(jq -r '.ss58PublicKey' ~/.address.json)

# Rich Text
BOLD='\033[1m'
ITALIC='\033[3m'
BOLD_WHITE='\033[97m'
BLUE='\033[1;34m'
GREEN='\033[1;32m'
RED='\033[3;31m'
YELLOW='\033[3;33m'
GREY='\033[3;30m'
STYLE_END='\033[0m'
LINK_START='\033]8;;https://faucet.polkadot.io/\033\\'
LINK_END='\033]8;;\033\\'

# Output message
echo "

Paseo Address: ${BOLD}${addr}${STYLE_END}
${BOLD}${RED}Note:${STYLE_END} ${GREY}Do not use this address for anything of real value${STYLE_END}

$(sh ~/check-balance.sh)
Paste the address into the ${LINK_START}${BLUE}paseo faucet${LINK_END}${STYLE_END} to receive tokens for testing your contracts!
"

# TODO! 
# 1. check for updates to the the Dockerfile for this devcontainer 
#    & emit notice to rebuild if their has been an update