use reqwest::Client;
use crate::markets::mpreis::MpreisBrowseResponse;
use crate::providers::Fetchable;

pub struct MpreisShopApi;

const MPREIS_API_URL: &str = "https://uzxors8tl2-dsn.algolia.net/1/indexes/prod_mpreis_8450/browse?X-Algolia-API-Key=6d27574257fd3a92542ff880585333f1&X-Algolia-Application-Id=UZXORS8TL2&X-Algolia-Agent=Vue.js";

impl Fetchable for MpreisShopApi {
    type ResponseImpl = MpreisBrowseResponse;

    async fn fetch(client: &Client) -> crate::error::Result<Self::ResponseImpl> {
        let mut total = MpreisBrowseResponse::default();

        loop {
            let url = match &total.cursor {
                Some(c) => format!("{}&cursor={}", MPREIS_API_URL, c),
                None => MPREIS_API_URL.to_string(),
            };

            let response = client.get(&url).send().await?;
            total += &mut response.json::<MpreisBrowseResponse>().await?;

            if total.cursor.is_none() {
                break;
            }
        }

        Ok(total)
    }
}
