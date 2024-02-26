use crate::providers::Merge;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlgoliaBrowseResponse<T> {
    hits: Vec<T>,

    /// The elapsed time for processing the browse request.
    #[serde(rename = "processingTimeMS")]
    processing_time_ms: u32,

    /// The cursor which can be used in a subsequent API call to continue browsing.
    pub(crate) cursor: Option<String>,
}

impl<T> Merge for AlgoliaBrowseResponse<T> {
    fn merge(&mut self, rhs: Self) {
        self.hits.extend(rhs.hits);
        self.processing_time_ms += rhs.processing_time_ms;
        self.cursor = rhs.cursor;
    }
}
