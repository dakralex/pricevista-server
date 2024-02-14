use crate::markets::billa::BillaSearchResponse;
use crate::markets::spar::SparSearchResponse;
use brotli::CompressorWriter;
use clap::ValueEnum;
use reqwest::Client;
use std::fs::File;
use std::io::Write;

const BILLA_API_URL: &str = "https://shop.billa.at/api/products/search/*?storeId=00-10";
const SPAR_API_URL: &str = "https://search-spar.spar-ics.com/fact-finder/rest/v5/search/products_lmos_at?query=*&q=*&page=1&hitsPerPage=1000";

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum FetchSourceType {
    All,
    Billa,
    HeissePreise,
    Spar,
}

pub enum FetchPersistType {
    Json,
    JsonBrotli,
}

async fn save_fetch(
    content: &String,
    source: FetchSourceType,
    target: FetchPersistType,
) -> std::io::Result<()> {
    let date = chrono::Utc::now().format("%Y-%m-%d");

    match target {
        FetchPersistType::Json => {
            let filename = format!("{:?}-{}.json", source, date);
            let mut output = File::create(filename)?;
            output.write_all(content.as_bytes())
        }
        FetchPersistType::JsonBrotli => {
            let filename = format!("{:?}-{}.json.br", source, date);
            let mut output = File::create(filename)?;
            let mut writer = CompressorWriter::new(&mut output, 4096, 11, 22);
            writer.write_all(content.as_bytes())
        }
    }
}

pub async fn fetch_billa() -> reqwest::Result<BillaSearchResponse> {
    let client = Client::new();
    let response = client.get(BILLA_API_URL).send().await?;
    let body = response.text().await?;
    save_fetch(&body, FetchSourceType::Billa, FetchPersistType::Json)
        .await
        .unwrap();
    let json = serde_json::from_str(&body).unwrap();
    Ok(json)
}

pub async fn fetch_spar() -> reqwest::Result<SparSearchResponse> {
    let client = Client::new();
    let response = client.get(SPAR_API_URL).send().await?;
    let body = response.text().await?;
    save_fetch(&body, FetchSourceType::Spar, FetchPersistType::Json)
        .await
        .unwrap();
    let json = serde_json::from_str(&body).unwrap();
    Ok(json)
}
