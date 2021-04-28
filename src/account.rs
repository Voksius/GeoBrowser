extern crate reqwest;

#[tokio::main]
pub async fn idlookup(acc_id: String, client: &reqwest::Client) -> Result<(), reqwest::Error> 
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
        let acc_gold   : &str = acc_data[5];
        let acc_silver : &str = acc_data[7];
        let acc_stars  : &str = acc_data[13];

        println!("Account name: {}", acc_name);
        println!("\n{} secret coins, {} user coins", acc_gold, acc_silver);
        println!("{} stars.", acc_stars);
    
    }

    Ok(())

}

#[tokio::main]
pub async fn list_levels(acc_id: String, client: &reqwest::Client) -> Result<(), reqwest::Error> 
{
    println!("yo");

    let params = [
        ("type", "5"),
        ("str", acc_id.trim()),
        ("secret", "Wmfd2893gb7")
    ];

    let res = client.post("http://boomlings.com/database/getGJLevels21.php")
        .form(&params)
        .send()
        .await?;

    let body = res.text().await?;

    println!("{:?}", body);

    Ok(())
}