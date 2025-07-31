addr=$(jq -r '.ss58PublicKey' .address.json)

# fetch account data
RAW=$(npx --yes @polkadot/api-cli \
          --ws wss://paseo-rpc.dwellir.com \
          query.system.account "$addr" --output json 2>/dev/null)

# Extract free balance value
BAL_PLANCK=$(echo "$RAW" | jq -r '.account.data.free')
BAL_PAS=$(awk "BEGIN {printf \"%.10f\", $BAL_PLANCK/1e10}")

# Rich text
GREEN='\033[1;32m'
RESET='\033[0m'

# Output message
echo "balance: ${GREEN}$BAL_PAS PAS${RESET}"
