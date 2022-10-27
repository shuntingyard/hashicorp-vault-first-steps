use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::error::ClientError;

// Can be used from bin under its crate name.
pub fn get_client() -> Result<VaultClient, ClientError> {
    // Create a client
    VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("http://127.0.0.1:8200")
            .token("root")
            .build()
            .expect("ClientSettingsBuilder failed!"), // Builder errors
    )
}
