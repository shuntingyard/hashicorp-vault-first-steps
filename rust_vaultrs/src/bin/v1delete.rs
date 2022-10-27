use tokio;
use vaultrs::kv1;

use rust_vaultrs::get_client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = get_client().expect("Client failed");

    // Instead of cmdl args:
    let mount = "kv-v1";
    let path = "my/secrets";

    kv1::delete(&client, &mount, &path).await.expect("DELETE failed");

    println!("Deleted {}", path);
}
