use crate::parser;
use clap::ValueEnum;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const BILLA_API_URL: &'static str = "https://shop.billa.at/api/products/search/*?storeId=00-10";

#[derive(Debug, Serialize, Deserialize)]
pub struct BillaSearchResponse {
    #[serde(rename = "queryId")]
    query_id: String,
    count: usize,
    total: usize,
    results: Vec<BillaSearchResult>,
    facets: Vec<BillaResultFacet>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BillaSearchResult {
    #[serde(rename = "ageRequiredInMonths")]
    age_required_in_months: Option<u32>,
    #[serde(with = "parser::double_as_string")]
    amount: f64,
    badges: Option<Vec<BillaBadge>>,
    brand: Option<BillaBrand>,
    category: Option<String>,
    #[serde(rename = "countryOfOrigin")]
    country_of_origin: Option<String>,
    #[serde(rename = "depositType")]
    deposit_type: Option<String>,
    #[serde(rename = "descriptionShort")]
    description_short: Option<String>,
    #[serde(rename = "descriptionLong")]
    description_long: Option<String>,
    #[serde(rename = "drainedWeight")]
    drained_weight: Option<f64>,
    images: Vec<String>,
    mapped_category: Option<String>,
    #[serde(rename = "maxQuantity")]
    max_quantity: Option<u32>,
    medical: bool,
    #[serde(rename = "minQuantity")]
    min_quantity: Option<u32>,
    name: String,
    #[serde(rename = "packageLabel")]
    package_label: Option<String>,
    #[serde(rename = "packageLabelKey")]
    package_label_key: Option<String>,
    #[serde(rename = "parentCategories")]
    parent_categories: Vec<Vec<BillaParentCategory>>,
    preparation_time: Option<u32>,
    price: BillaPriceInfo,
    #[serde(rename = "productId")]
    product_id: String,
    product_marketing: Option<String>,
    purchased: bool,
    #[serde(rename = "quantityStepSize")]
    quantity_step_size: Option<u32>,
    sku: String,
    slug: String,
    #[serde(rename = "upsellSku")]
    upsell_sku: Option<String>,
    #[serde(rename = "volumeLabelKey")]
    volume_label_key: String,
    #[serde(rename = "volumeLabelLong")]
    volume_label_long: String,
    #[serde(rename = "volumeLabelShort")]
    volume_label_short: Option<String>,
    weight: f64,
    #[serde(rename = "weightArticle")]
    weight_article: bool,
    #[serde(rename = "weightPerPiece")]
    weight_per_piece: f64,
    #[serde(rename = "weightPieceArticle")]
    weight_piece_article: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct BillaBrand {
    name: String,
    slug: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct BillaParentCategory {
    key: Option<String>,
    name: String,
    slug: String,
    #[serde(rename = "orderHint")]
    order_hint: Option<String>,
}

/// Enumeration of the different badges used in the BILLA Online Shop.
///
/// These badges are used to indicate some quality of a product, that is either
/// some private or public sign of quality (e.g. FairTrade, Rainforest, DOP),
/// a hint for a specific diet (e.g. vegan, gluten-free), a hint for how the
/// product should be stored (e.g. deep-frozen) or something more ambigous
/// (e.g. new product, product from Austria).
#[derive(Debug, Serialize, Deserialize)]
enum BillaBadge {
    /// Austrian "AMA Gütesiegel" organic food certification
    #[serde(rename = "gs-amabio")]
    AMAMark,

    /// French/Swiss "Appellation d’Origine Contrôlée" certification
    #[serde(rename = "gs-aoc")]
    AOCMark,

    /// French/Swiss "Appellation d’Origine Protégée" certification
    #[serde(rename = "gs-aop")]
    AOPMark,

    /// Austrian "ARGE Heumilch" certification
    #[serde(rename = "gs-argeheu")]
    ArgeMark,

    /// International Aquaculture Stewardship Council certification
    #[serde(rename = "gs-asc")]
    ASCMark,

    /// (Generic?) organic food certification
    #[serde(rename = "gs-bio")]
    BioMark,

    /// International Demeter organic food certification
    #[serde(rename = "gs-demeter")]
    DemeterMark,

    /// Italian "Denominazione di'Origine controllata" certification
    #[serde(rename = "gs-doc")]
    DOCMark,

    /// Italian "Denominazione di'Origine Protetta" certification
    #[serde(rename = "gs-dop")]
    DOPMark,

    /// European Union's organic food certification
    #[serde(rename = "gs-eubio")]
    EUBioMark,

    /// International Forest Stewardship Council certification
    #[serde(rename = "gs-fsc")]
    FSCMark,

    /// International FairTrade certification
    #[serde(rename = "gs-ft")]
    FairTradeMark,

    /// Generic certification for products that are free from genetic engineering
    #[serde(rename = "gs-gentech")]
    GenTechMark,

    /// European "Protected Geographical Indication" certification
    ///
    /// - German: geschützte geografische Angabe (ggA)
    /// - French: indication géographique protégée (IGP)
    /// - Italian: indicazione geografica protetta (IGP)
    /// - Portuguese: indicação geográfica protegida (IGP)
    /// - Spanish: indicación geográfica protegida (IGP)
    #[serde(rename = "gs-gga")]
    PGIMark,

    /// European "Traditional Speciality Guaranteed" certification
    ///
    /// - German: garantiert traditionelle Spezialität (gtS)
    /// - French: spécialité traditionnelle garantie (STG)
    /// - Italian: specialità tradizionale garantita (STG)
    /// - Spanish: especialidad tradicional garantizada (ETG)
    ///
    /// Note: The Billa API has probably an spelling error ("gta" instead of "gts")
    #[serde(rename = "gs-gta")]
    TSGMark,

    /// European "Protected Designation of Origin" certification
    ///
    /// - German: geschützte Ursprungsbezeichnung (gU)
    /// - French: Appellation d’Origine Contrôlée (AOP)
    /// - Italian: Denominazione d’Origine Protetta (DOP)
    /// - Portuguese: Denominação de Origem Protegida (DOP)
    /// - Spanish: denominación de origen protegida (DOP)
    #[serde(rename = "gs-gu")]
    PDOMark,

    /// International Marine Stewardship Council certification
    #[serde(rename = "gs-msc")]
    MSCMark,

    /// Proprietary PRO PLAN pet food certification
    #[serde(rename = "gs-proplan")]
    ProPlanMark,

    /// International Rainforest Alliance certification
    #[serde(rename = "gs-rainfor")]
    RainforestMark,

    /// International UTZ Certified certification
    #[serde(rename = "gs-utz")]
    UTZCertifiedMark,

    /// Badge for vegan products
    ///
    /// This badge is an alternative to [BillaBadge::VeganProduct].
    #[serde(rename = "gs-vegan")]
    VeganMark,

    /// Label for products originating from Austria
    #[serde(rename = "homecountry")]
    AustrianProduct,

    /// Label for organic products
    ///
    /// This badge is an alternative to [BillaBadge::BioMark].
    #[serde(rename = "pp-bio")]
    BioProduct,

    /// Label for cooled products
    #[serde(rename = "kuehlung")]
    CooledProduct,

    /// In-store label for deep frozen products
    #[serde(rename = "tiefkuehlung")]
    DeepFrozenProduct,

    /// Label for gluten-free products
    #[serde(rename = "pp-glutenfr")]
    GlutenFreeProduct,

    /// Label for lactose-free products
    #[serde(rename = "pp-lakto")]
    LactoseFreeProduct,

    /// In-store label for new products
    #[serde(rename = "eg-neu")]
    NewProduct,

    /// Label for vegan products
    ///
    /// This badge is an alternative to [BillaBadge::VeganMark].
    #[serde(rename = "pp-vegan")]
    VeganProduct,

    /// Label for vegetarian products
    #[serde(rename = "pp-veget")]
    VegetarianProduct,
}

#[derive(Debug, Serialize, Deserialize)]
struct BillaPriceInfo {}

#[derive(Debug, Serialize, Deserialize)]
struct BillaResultFacet {
    key: String,
    label: String,
    #[serde(rename = "isMultiSelect")]
    is_multi_select: bool,
    terms: Option<Vec<BillaFacetTerm>>,
    #[serde(rename = "comparisonTerms")]
    comp_terms: Option<Vec<BillaFacetTerm>>,
    range: Option<BillaFacetRange>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BillaFacetTerm {
    key: String,
    label: String,
    term: String,
    count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct BillaFacetRange {
    key: String,
    label: String,
    count: usize,
    #[serde(rename = "labelSymbol")]
    label_symbol: String,
    min: f64,
    max: f64,
}

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq)]
pub enum FetchSourceType {
    All,
    Billa,
    HeissePreise,
}

pub async fn fetch_billa() -> Result<BillaSearchResponse, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get(BILLA_API_URL)
        .send()
        .await?
        .json::<BillaSearchResponse>()
        .await?;

    Ok(response)
}
