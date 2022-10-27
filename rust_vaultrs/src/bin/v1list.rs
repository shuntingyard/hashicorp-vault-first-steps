use tokio;
use vaultrs::kv1;

use rust_vaultrs::get_client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = get_client().expect("Client failed");

    // Instead of cmdl args:
    let mount = "kv-v1";

    let secrets = kv1::list(&client, &mount, "dev/config")
        .await
        .expect("list failed");

    println!("secrets {:?}", secrets);
    println!("secrets.data {:?}", secrets.data);
    println!("secrets.data.keys {:?}", secrets.data.keys);
}
