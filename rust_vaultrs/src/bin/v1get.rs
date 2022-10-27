use std::collections::HashMap;

use tokio;
use vaultrs::kv1;

// Imported from my own lib here.
use rust_vaultrs::get_client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = get_client().expect("Client failed");

    // Instead of cmdl args:
    let mount = "kv-v1";
    let path = "eng/apikey/Google";

    let secrets: HashMap<String, String> =
        kv1::get(&client, &mount, &path).await.expect("GET failed");

    println!("{:?}", secrets);
}
