extern crate reqwest;

#[tokio::main]
pub async fn lookup(acc_id: String, client: &reqwest::Client) -> Result<(), reqwest::Error> 
{

    let params = [
        ("targetAccountID", acc_id.trim()),
        ("secret", "Wmfd2893gb7")
    ];

    let res = client.post("http://boomlings.com/database/getGJUserInfo20.php")
        .form(&params)
        .send()
        .await?;

    let body = res.text().await?;
    
    if body == "-1"
    {
        println!("That account couldn't be found.");
    }

    else
    {
        println!("{:?}\n", body);

        let acc_data: Vec<&str> = body.split(":").collect();

        let acc_name   : &str = acc_data[1];

        println!("Account name: {}", acc_name);
    }

    Ok(())

}
