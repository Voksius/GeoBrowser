#[tokio::main]
pub async fn lookup(mut id: String, client: &reqwest::Client) -> Result<(), reqwest::Error> {

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
    println!("{}", body);

    Ok(())
}