use crate::providers::Merge;
use serde::Deserialize;
use std::collections::HashMap;

/// A search response for the FACT-Finder Search REST API.
///
/// This struct is usually created by querying the REST API of any FACT-Finder
/// application with the endpoint `/fact-finder/rest/v5/search` and
/// deserializing the JSON response with this struct. The generic type [T] is
/// used for the application-specific [SearchRecord] result items.
///
/// This implementation is incomplete as the implemented markets that use this
/// software do not use the other capabilities or they are not used by
/// PriceVista.
///
/// For more information see the
/// [official demo documentation](https://ng-demo.fact-finder.de/fact-finder/swagger-ui.html#/search/searchUsingGET).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse<T> {
    /// The list of [SearchFacet] which are available for the search records.
    facets: Vec<SearchFacet>,

    /// The field to role mapping, which provides additional metadata to the
    /// fields of the resulting [SearchRecord<T>].
    field_roles: HashMap<String, String>,

    /// The linking parameter for a follow-up search request, which can improve
    /// the request performance instead of opening a new search query request.
    pub(crate) follow_search: Option<String>,

    /// The list of [SearchRecord] that were found relevant for the search query.
    hits: Vec<SearchRecord<T>>,

    /// The metadata how the [SearchResponse] is paged and navigated.
    pub(crate) paging: PagingMetadata,

    /// Whether the search query timed out while processing.
    ///
    /// If the search query timed out during processing, the resulting hits
    /// might not be accurate or contain all relevant products and therefore
    /// the request should be repeated.
    timed_out: bool,

    /// The elapsed time in milliseconds to process the search query for loop54 personalization.
    took_loop_54: u32,

    /// The elapsed time in milliseconds to process the search query in the framework.
    took_total: u32,

    /// The elapsed time in milliseconds to process the search query in the core.
    took_worldmatch: u32,

    /// Total amount of search result items.
    total_hits: usize,
}

impl<T> Default for SearchResponse<T> {
    fn default() -> Self {
        SearchResponse {
            facets: vec![],
            field_roles: HashMap::default(),
            follow_search: None,
            hits: vec![],
            paging: PagingMetadata::default(),
            timed_out: false,
            took_loop_54: 0,
            took_total: 0,
            took_worldmatch: 0,
            total_hits: 0,
        }
    }
}

impl<T> Merge for SearchResponse<T> {
    fn merge(&mut self, rhs: Self) {
        self.facets.extend(rhs.facets);
        self.field_roles.extend(rhs.field_roles);
        self.follow_search = rhs.follow_search;
        self.hits.extend(rhs.hits);
        self.paging = rhs.paging;
        self.timed_out &= rhs.timed_out;
        self.took_loop_54 += rhs.took_loop_54;
        self.took_total += rhs.took_total;
        self.took_worldmatch += rhs.took_worldmatch;
        self.total_hits += rhs.total_hits;
    }
}

/// A FACT-Finder search result item struct, which represents a single found
/// record inside of the channel cluster, that is the catalog of the application.
///
/// The generic type [T] is used for the application-specific field values.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchRecord<T> {
    /// The list of words that caused this [SearchRecord] to be returned.
    #[serde(rename = "foundWords")]
    found_words: Vec<String>,

    /// The id of the [SearchRecord].
    id: String,

    /// The application-specific field values of the [SearchRecord].
    #[serde(rename = "masterValues")]
    master_values: T,

    /// The position of the [SearchRecord] inside an outer list.
    position: u32,

    /// The score how well the [SearchRecord] matches the query parameters.
    score: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchFacet {
    /// The field in the [SearchRecord] which is represented by the [SearchFacet].
    associated_field_name: String,

    /// The number of digits for decimals in a Slider [SearchFacet].
    decimal_places: Option<u32>,

    /// The amount of links to be displayed in the selection menu.
    detailed_links: u32,

    /// The type of the elements.
    #[serde(rename = "type")]
    element_type: Option<FacetElementType>,

    /// The elements of the [SearchFacet] that are available.
    elements: Vec<FacetElement>,

    /// The type of the [SearchFacet].
    filter_style: String,

    /// The display name of the [SearchFacet].
    name: Option<String>,

    /// The elements of the [SearchFacet] that are currently selected.
    selected_elements: Vec<FacetElement>,

    /// The type of how the [SearchFacet] behaves when elements are selected.
    selection_type: FacetSelectionType,

    /// Whether preview images should be shown.
    show_preview_images: bool,

    /// The units which should be shown to represents a [FacetElement].
    unit: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FacetElement {
    absolute_max_value: Option<f64>,
    absolute_min_value: Option<f64>,
    cluster_level: u32,
    selected: String,
    selected_max_value: Option<f64>,
    selected_min_value: Option<f64>,
    text: String,
    total_hits: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum FacetFilterStyle {
    Default,
    Slider,
    Multiselect,
    Tree,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum FacetSelectionType {
    SingleHideUnselected,
    SingleShowUnselected,
    MultiSelectOr,
    MultiSelectAnd,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum FacetElementType {
    Alphanumeric,
    Boolean,
    CategoryPath,
    Date,
    Float,
    Geo,
    Gtin,
    Integer,
    Multi,
    Property,
    Text,
}

/// A FACT-Finder search paging metadata, which provides the metadata how the
/// search result is paged and how to navigate it further.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagingMetadata {
    current_page: u32,
    default_hits_per_page: u32,
    hits_per_page: u32,
    pub(crate) next_link: Option<PageLink>,
    page_count: u32,
    previous_link: Option<PageLink>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageLink {
    current_page: bool,
    pub(crate) number: u32,
}
