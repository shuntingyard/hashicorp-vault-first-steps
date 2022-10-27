# ...

## My First Steps
### Installation
trivial

### First Steps
Tutorial used: [Static Secrets: Key/Value Secrets Engine](https://learn.hashicorp.com/tutorials/vault/static-secrets)
(important note: this is about KV engine version 1 which doesn't keep versions
of values entered).

All files created have names starting with `first_start*` (repo root and curl
subdirectory).

Start vault, set environment, mount, etc...
```zsh
./first_start.sh
source first_start.setenv
./first_start_enable_kvv1.sh
curl/first_start_vault_kv_post_googleapi.sh
...
...
./first_start_killvault.sh
```
Rust implementation of "multiple values" (mongodb) exercise in `rust_vaultrs`.

- [ ] Does the API allow to list keys present in a KV v1 vault?

### First Steps
Tutorial used: [Versioned Key/Value Secrets Engine](https://learn.hashicorp.com/tutorials/vault/versioned-kv?in=vault/secrets-management)

The first and most important reason for a store with versioning is the ability
to recover from unintentional data loss or overwrite.

## Resources
### Vault Manuals and Intros Used


### Programming
[Python hvac API Usage](https://hvac.readthedocs.io/en/stable/usage/)

[Crate hashicorp_vault (Github)](https://github.com/ChrisMacNaughton/vault-rs)
