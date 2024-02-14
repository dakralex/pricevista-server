use crate::markets::billa::BillaSearchResponse;
use crate::markets::spar::SparSearchResponse;
use clap::ValueEnum;
use reqwest::Client;

const BILLA_API_URL: &'static str = "https://shop.billa.at/api/products/search/*?storeId=00-10";
const SPAR_API_URL: &'static str = "https://search-spar.spar-ics.com/fact-finder/rest/v5/search/products_lmos_at?query=*&q=*&page=1&hitsPerPage=1000";

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum FetchSourceType {
    All,
    Billa,
    HeissePreise,
    Spar,
}

pub async fn fetch_billa() -> Result<BillaSearchResponse, reqwest::Error> {
    let client = Client::new();
    let response = client.get(BILLA_API_URL).send().await?;

    let response_json = response.json::<BillaSearchResponse>().await?;

    Ok(response_json)
}

pub async fn fetch_spar() -> Result<SparSearchResponse, reqwest::Error> {
    let client = Client::new();
    let response = client.get(SPAR_API_URL).send().await?;

    let response_json = response.json::<SparSearchResponse>().await?;

    Ok(response_json)
}
