#[tokio::main]
pub async fn lookup(id: String, client: &reqwest::Client) -> Result<(), reqwest::Error> {

   let id_str: &str = id.trim();

   let zetex: bool = false;

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

    if body == "-1"
    {
        println!("Level could not be found.");

        if zetex == true {
         
            webbrowser::open("https://www.youtube.com/watch?v=yfGql6suU-c");

        }
    } 
    
    else 
    {

    let lvl_data: Vec<&str> = body.split(":").collect();

    let lvl_id    : &str = lvl_data[1];
    let lvl_name  : &str = lvl_data[3];
    let lvl_desc  : &str = lvl_data[5];
    let lvl_author: &str = lvl_data[11];
    let lvl_likes : &str = lvl_data[23];

    let desc_vector: Vec<u8> = base64::decode(lvl_desc).unwrap();
    let desc_readable = std::str::from_utf8(&desc_vector).unwrap();

    println!("Level Name: {}", lvl_name);
    println!("Level ID: {}", lvl_id);
    println!("Author's Player ID: {}", lvl_author);

    println!("\n{}\n", desc_readable);

    println!("{} likes.", lvl_likes);

    }

    Ok(())

}
