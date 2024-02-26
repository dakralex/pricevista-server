use reqwest::Client;
use crate::markets::billa::BillaSearchResponse;
use crate::providers::Fetchable;

pub struct BillaShopApi;

const BILLA_API_URL: &str = "https://shop.billa.at/api/products/search/*?storeId=00-10";

impl Fetchable for BillaShopApi {
    type ResponseImpl = BillaSearchResponse;

    async fn fetch(client: &Client) -> crate::error::Result<Self::ResponseImpl> {
        let response = client.get(BILLA_API_URL).send().await?;
        let body = response.text().await?;
        let data = serde_json::from_str(&body).unwrap();
        Ok(data)
    }
}
