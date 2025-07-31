

# subkey generate --network polkadot --output-type json > address.json
addr=$(jq -r '.ss58PublicKey' address.json)

# ── colour / style codes ────────────────────────────────────────────────────────
BOLD_WHITE='\033[1;97m'       # bright-white + bold  (for the address)
BLUE='\033[1;34m'             # bright blue          (for the link text)
GREEN='\033[1;32m'             # bright green         (for success messages)
RESET='\033[0m'

# ── OSC-8 hyperlink wrapper ─────────────────────────────────────────────────────
#   ESC ] 8 ; ; URL ESC \   …link text…   ESC ] 8 ; ; ESC \
LINK_START='\033]8;;https://faucet.polkadot.io/\033\\'
LINK_END='\033]8;;\033\\'

# ── output ──────────────────────────────────────────────────────────────────────
echo "

Paseo Address: ${BOLD_WHITE}${addr}${RESET}
$(sh ./check-balance.sh)

Paste it into the ${LINK_START}${BLUE}paseo faucet${LINK_END}${RESET} to receive tokens for testing your contracts!

"

