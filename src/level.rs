#[tokio::main]
pub async fn lookup(id: String) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

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