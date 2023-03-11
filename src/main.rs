use scraper::{Html,Selector};
use std::error::Error;
use csv::Writer;
use chrono::prelude::*;

#[derive(serde::Serialize)]
struct Prices{
    lz_houston: f32, 
    lz_north: f32, 
    lz_south: f32, 
    lz_west: f32
}


#[tokio::main]
async fn main(){

    let url = create_local_date_url();
    print!("{}", url);

    let response = call_api(url).await;
    match response {
        Ok(res) => write_csv(&res.to_owned(), "prices.csv".to_string()),
        Err(e) => {Ok(print!("Could not reach api: {}", e))}
    };

}

fn create_local_date_url() -> String {
    let local_date_time = Local::now();

    let month = local_date_time.month().to_string();
    let day = local_date_time.day().to_string();
    let year = local_date_time.year().to_string();

    let front_url = "https://www.ercot.com/content/cdr/html/";
    let back_url = "_dam_spp.html";

    let full_url = front_url.to_owned() + &year + &month + &day + back_url;

    full_url
}

fn write_csv(response: &str, path: String) -> Result<(), Box<dyn Error>> {

    let document = Html::parse_document(&response);
    let selector = Selector::parse("tr").unwrap();

    let mut wtr = Writer::from_path(path)?;


    for element in document.select(&selector){

        let row = element.text().collect::<Vec<_>>();

        let filtered_row = row.iter()
            .filter(|&&i| i != "\n\t\t" && i != "\n\t")
            .collect::<Vec<_>>();

        if filtered_row[0] != &"Oper Day" {
            wtr.serialize(Prices{
                lz_houston : filtered_row[11].parse::<f32>().unwrap(),
                lz_north : filtered_row[13].parse::<f32>().unwrap(),
                lz_south : filtered_row[15].parse::<f32>().unwrap(),
                lz_west : filtered_row[16].parse::<f32>().unwrap()
            })?;
        }
    }

    wtr.flush()?;

    Ok(())
}


async fn call_api(url: String) -> Result<String, reqwest::Error>{

    let res = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

