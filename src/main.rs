use scraper::{Html,Selector};
use std::error::Error;
use csv::Writer;
use chrono::prelude::*;
// use std::env;

#[derive(serde::Serialize)]
struct Prices{
    lz_houston: f32, 
    lz_north: f32, 
    lz_south: f32, 
    lz_west: f32
}

#[tokio::main]
async fn main(){

    //retrieve and create a get request url for the Local date
    let url = create_local_date_url();

    //retrieve DAM prices in html format from ERCOT api
    let response = call_ercot_api(url).await;
    let response = match response {
        Ok(res) => res,
        Err(error) => panic!("Could not reach Api: {}", error)
    };

    //Parse HTML and write desired prices to csv file
    let write = write_csv(&response.to_owned(), "prices.csv".to_string());
    match write {
        Ok(res) => res,
        Err(error) => panic!("Could not write to csv: {}", error)
    };


}

fn create_local_date_url() -> String {
    let local_date_time = Local::now();

    let month = "0".to_owned() + &local_date_time.month().to_string();
    let day = local_date_time.day().to_string();
    let year = local_date_time.year().to_string();

    let front_url = "https://www.ercot.com/content/cdr/html/";
    let back_url = "_dam_spp.html";

    let full_url = front_url.to_owned() + &year + &month + &day + back_url;

    full_url
}

fn write_csv(response: &str, path: String) -> Result<(), Box<dyn Error>> {

    //parse html document
    let document = Html::parse_document(&response);
    let selector = Selector::parse("tr").unwrap();

    //create a csv writr
    let mut wtr = Writer::from_path(path)?;

    //loop through every row in prices table
    for element in document.select(&selector){

        let row = element.text().collect::<Vec<_>>();

        //filter out an extra indicies that are not useful data
        let filtered_row = row.iter()
            .filter(|&&i| i != "\n\t\t" && i != "\n\t")
            .collect::<Vec<_>>();

        //add each serialized struct row to the csv writer
        if filtered_row[0] != &"Oper Day" {
            wtr.serialize(Prices{
                lz_houston : filtered_row[11].parse::<f32>().unwrap(),
                lz_north : filtered_row[13].parse::<f32>().unwrap(),
                lz_south : filtered_row[15].parse::<f32>().unwrap(),
                lz_west : filtered_row[16].parse::<f32>().unwrap()
            })?;
        }
    }

    //write csv file
    wtr.flush()?;

    Ok(())
}


async fn call_ercot_api(url: String) -> Result<String, reqwest::Error>{

    //make a get request to the ercot dam api
    let res = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

#[cfg(test)]
mod test;