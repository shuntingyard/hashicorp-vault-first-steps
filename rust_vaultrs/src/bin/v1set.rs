use std::collections::HashMap;

use tokio;
use vaultrs::kv1;

use rust_vaultrs::get_client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = get_client().expect("Client failed");

    // Instead of cmdl args:
    let mount = "kv-v1";
    let path = "dev/config/mongodb";
    let my_secrets = HashMap::from([
        ("url", "foo.example.com:35533"),
        ("db_name", "users"),
        ("user", "lausberg"),
        ("password", "3lemente6er1iterarischen4hetorik"),
    ]);

    kv1::set(&client, &mount, &path, &my_secrets)
        .await
        .expect("POST failed");

    let secrets: HashMap<String, String> =
        kv1::get(&client, &mount, &path).await.expect("GET failed");

    println!("{:?}", secrets);
}
