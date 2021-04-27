use std::io::stdin;

mod level;

fn main() {

    let mut lvl_id: String = String::new();

    println!("Enter the level ID..");

    stdin()
        .read_line(&mut lvl_id)
        .expect("Line read error");

    lvl_id.pop();

    level::lookup(lvl_id);
}