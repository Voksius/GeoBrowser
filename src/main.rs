use std::io::stdin;
use reqwest::Client;
mod level;

fn main() {

    let client: Client = Client::new();
    let mut lvl_id: String = String::new();

    println!("Enter the level ID..");

    stdin()
        .read_line(&mut lvl_id)
        .expect("Line read error");

    level::lookup(lvl_id, client);
}