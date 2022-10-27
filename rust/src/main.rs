// Afaik this crate only works with KV v2.
use hashicorp_vault::Client;

fn main() {
    let client = Client::new("http://localhost:8200", "root").unwrap();

    match client.get_secret("...") {
        Ok(secret) => {
            println!("{}", secret);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
