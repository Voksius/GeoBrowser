#[tokio::main]
pub async fn lookup(mut id: String, client: reqwest::Client) -> Result<(), reqwest::Error> {

    if id.ends_with('\n')
    {
        id.pop();
    }

    println!("Fetching data for {}", id);

    id.pop();

    let params = [
        ("secret", "Wmfd2893gb7"),
        ("levelID", id.as_str())
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