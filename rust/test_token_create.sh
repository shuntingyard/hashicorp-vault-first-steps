export VAULT_ADDR=http://localhost:8200
export VAULT_TOKEN=`read -p "root token from server startup: " T; echo "$T"`
vault token create -id="test12345" -ttl="5m"
