use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default)]
pub struct FilterOptions {
    #[serde(rename = "q")]
    pub query: Option<String>,
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchArticleSchema {
    pub store_ids: Option<Vec<usize>>,
    pub price_min: Option<f64>,
    pub price_max: Option<f64>,
}
