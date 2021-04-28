#[tokio::main]
pub async fn lookup(id: String, client: &reqwest::Client) -> Result<(), reqwest::Error> {

   let id_str: &str = id.trim();

    println!("Fetching data for {}", id);


    let params = [
        ("secret", "Wmfd2893gb7"),
        ("levelID", id_str)
    ];

    let res = client.post("http://boomlings.com/database/downloadGJLevel22.php")
        .form(&params)
        .send()
        .await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;
    let lvl_data: Vec<&str> = body.split(":").collect();

    println!("{}\n{:?}\n", body, lvl_data);

    let lvl_id  : &str = lvl_data[1];
    let lvl_name: &str = lvl_data[3];

    println!("Level Name: {}", lvl_name);
    println!("Level ID: {}", lvl_id);

    Ok(())
}