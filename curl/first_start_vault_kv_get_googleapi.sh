curl --header "X-Vault-Token: $VAULT_TOKEN" \
    $VAULT_ADDR/v1/kv-v1/eng/apikey/Google \
    | jq ".data"
