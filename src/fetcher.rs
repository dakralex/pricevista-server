use crate::markets::billa::BillaSearchResponse;
use clap::ValueEnum;
use reqwest::Client;

const BILLA_API_URL: &'static str = "https://shop.billa.at/api/products/search/*?storeId=00-10";

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum FetchSourceType {
    All,
    Billa,
    HeissePreise,
}

pub async fn fetch_billa() -> Result<BillaSearchResponse, reqwest::Error> {
    let client = Client::new();
    let response = client.get(BILLA_API_URL).send().await?;

    let response_json = response.json::<BillaSearchResponse>().await?;

    Ok(response_json)
}
