use scraper::{Html,Selector};


#[tokio::main]
async fn main(){
    call_api().await;
}

struct Row{
    LZ_HOUSTON: f32, 
    LZ_NORTH: f32, 
    LZ_SOUTH: f32, 
    LZ_WEST: f32
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

    let selector = Selector::parse("tr").unwrap();

    for element in document.select(&selector){
        let row = element.text().collect::<Vec<_>>();

        let filtered_row = row.iter()
            .filter(|&&i| i != "\n\t\t" && i != "\n\t")
            .collect::<Vec<_>>();

        println!("{:?}", filtered_row);
    }

    // print!("{:?}", res);

    Ok(())
}

