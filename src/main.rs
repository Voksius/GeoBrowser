use std::io::stdin;
use reqwest::Client;
mod level;
mod account;

#[allow(unused_must_use)]
fn main() {

    let client: Client = Client::new();

    let mut zetex: bool = false;

    loop
    {

        println!("\n0. Exit");
        println!("1. Search Level by ID");
        println!("2. Search Account by ID");
        println!("3. Search Levels from Player ID");
        println!("What do you want to do?");

        let mut choice: String = String::new();

        stdin()
            .read_line(&mut choice)
            .expect("Line read error.");

        if choice.trim() == "zetex" 
        { 
            zetex = true;
        }

        if choice.trim() == "0" { break; }

        if choice.trim() == "1"
        {

            let mut lvl_id: String = String::new();

            println!("Enter the level ID..");

            stdin()
                .read_line(&mut lvl_id)
                .expect("Line read error");

            level::lookup(lvl_id, &client, zetex);
            
        }

        if choice.trim() == "2"
        {
            let mut acc_id: String = String::new();

            println!("Enter the account ID..");

            stdin()
                .read_line(&mut acc_id)
                .expect("Line read error");

            account::idlookup(acc_id, &client);

        }

        if choice.trim() == "3"
        {
            let mut acc_id:   String = String::new();
            let mut page_num: String = String::new();

            println!("Enter the account ID.");

            stdin()
                .read_line(&mut acc_id)
                .expect("Line read error.");

            println!("Enter the page number. Pages start at 0");

            stdin()
                .read_line(&mut page_num)
                .expect("Line read error.");

            account::list_levels(acc_id, page_num, &client);
        }
    }
}
