use reqwest::Client;
use crate::markets::spar::SparSearchResponse;
use crate::providers::Fetchable;

pub struct SparShopApi;

const SPAR_API_URL: &str = "https://search-spar.spar-ics.com/fact-finder/rest/v5/search/products_lmos_at?query=*&q=*&page=1&hitsPerPage=1000";

impl Fetchable for SparShopApi {
    type ResponseImpl = SparSearchResponse;

    async fn fetch(client: &Client) -> crate::error::Result<Self::ResponseImpl> {
        let response = client.get(SPAR_API_URL).send().await?;
        let body = response.text().await?;
        let data = serde_json::from_str(&body).unwrap();
        Ok(data)
    }
}