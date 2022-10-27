# In case we first ran
# `vault kv put kv-v1/eng/apikey/Google key=AAaaBBccDDeeOTXzSMT1234BB_Z8JzG7JkSVxI`
#
# this will overwrite with a value changing case in token:
#
curl --header "X-Vault-Token: $VAULT_TOKEN" \
    --request POST \
    --data '{"key": "aaAAbbCCddEEotxZsmt1234bb_z8jZg7jKsvXi"}' \
    $VAULT_ADDR/v1/kv-v1/eng/apikey/Google
