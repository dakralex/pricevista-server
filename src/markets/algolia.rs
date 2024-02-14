use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlgoliaBrowseResponse<T> {
    hits: Vec<T>,

    /// The elapsed time for processing the browse request.
    #[serde(rename = "processingTimeMS")]
    processing_time_ms: u32,
    query: String,
    params: String,

    /// The cursor which can be used in a subsequent API call to continue browsing.
    cursor: String,
}
