use std::io::stdin;
use reqwest::Client;
mod level;

#[allow(unused_must_use)]
fn main() {

    let client: Client = Client::new();

    loop
    {

        println!("0. Exit");
        println!("1. Search Level by ID");
        println!("What do you want to do?");

        let mut choice: String = String::new();

        stdin()
            .read_line(&mut choice)
            .expect("Line read error.");

        if choice.trim() == "0" { break; }

        if choice.trim() == "1"
        {

            let mut lvl_id: String = String::new();

            println!("Enter the level ID..");

            stdin()
                .read_line(&mut lvl_id)
                .expect("Line read error");

            level::lookup(lvl_id, &client);
            
        }
    }
}