


#[tokio::main]
async fn main(){
    call_api().await;
}

async fn call_api() -> Result<(),reqwest::Error>{
    let url = "https://www.ercot.com/content/cdr/html/20230308_dam_spp.html";

    let res = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    print!("{:?}", res);

    Ok(())
}

