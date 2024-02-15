use serde::{Deserialize, Serialize};
use std::ops::AddAssign;

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

impl<'a, T> AddAssign<&'a mut AlgoliaBrowseResponse<T>> for AlgoliaBrowseResponse<T> {
    fn add_assign(&mut self, rhs: &'a mut Self) {
        self.processing_time_ms += rhs.processing_time_ms;
        self.cursor = rhs.cursor.take();
        self.hits.append(&mut rhs.hits);
    }
}
