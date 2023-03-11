use scraper::{Html,Selector};


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

    let document = Html::parse_document(&res);

    let selector = Selector::parse("td").unwrap();

    for element in document.select(&selector){
        let m = element.text().last();
        println!("{:?}", m);
    }

    // print!("{:?}", res);

    Ok(())
}

