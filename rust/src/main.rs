use hashicorp_vault::Client;

fn main() {
    let client = Client::new("http://localhost:8200", "root").unwrap();

    match client.get_secret("/kv-v1/prod/cert/mysql") {

        Ok(secret) => {
            println!("{}", secret);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
