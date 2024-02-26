use crate::parser;
use crate::providers::{Fetch, Merge};
use reqwest::Client;
use serde::Deserialize;
use std::fmt::format;

pub struct BillaShopApi;

impl BillaShopApi {
    async fn request(client: &Client, page: usize) -> crate::error::Result<BillaSearchResponse> {
        let url = format!("{}/products", Self::API_BASE_URL);
        let page = format!("{}", page);

        let request = client.get(&url).query(&[
            ("page", page.as_str()),
            ("pageSize", "500"),
            ("storeId", "00-10"),
        ]);
        let response = request.send().await?;
        Ok(response.json::<BillaSearchResponse>().await?)
    }
}

impl Fetch for BillaShopApi {
    type ResponseImpl = BillaSearchResponse;
    const API_BASE_URL: &'static str = "https://shop.billa.at/api";

    async fn fetch(client: &Client) -> crate::error::Result<Self::ResponseImpl> {
        let mut page = 0;
        let mut total = Self::request(&client, page).await?;

        while &total.total > &0 {
            page += 1;
            total.merge(Self::request(&client, page).await?);
        }

        Ok(total)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillaSearchResponse {
    count: usize,
    total: usize,
    results: Vec<BillaProduct>,
    facets: Vec<BillaResultFacet>,
}

impl Merge for BillaSearchResponse {
    fn merge(&mut self, rhs: Self) {
        self.count += rhs.count;
        assert_eq!(self.total, rhs.total);
        self.results.extend(rhs.results);
        // TODO Make facets a HashSet or reimplement Extend/Trait on it
        self.facets.extend(rhs.facets);
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillaProduct {
    age_required_in_months: Option<u32>,
    #[serde(with = "parser::double_as_string")]
    amount: f64,
    badges: Option<Vec<BillaBadge>>,
    pub brand: Option<BillaBrand>,
    category: Option<String>,
    country_of_origin: Option<String>,
    deposit_type: Option<String>,
    description_short: Option<String>,
    description_long: Option<String>,
    drained_weight: Option<f64>,
    images: Vec<String>,
    mapped_category: Option<String>,
    max_quantity: Option<u32>,
    medical: bool,
    min_quantity: Option<u32>,
    pub name: String,
    package_label: Option<String>,
    package_label_key: Option<String>,
    parent_categories: Vec<Vec<BillaParentCategory>>,
    preparation_time: Option<u32>,
    pub price: BillaPriceInfo,
    product_id: String,
    product_marketing: Option<String>,
    purchased: bool,
    quantity_step_size: Option<u32>,
    sku: String,
    slug: String,
    upsell_sku: Option<String>,
    volume_label_key: String,
    volume_label_long: String,
    volume_label_short: Option<String>,
    weight: f64,
    weight_article: bool,
    weight_per_piece: f64,
    weight_piece_article: bool,
}

#[derive(Debug, Deserialize)]
struct BillaBrand {
    name: String,
    slug: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BillaParentCategory {
    key: Option<String>,
    name: String,
    slug: String,
    order_hint: Option<String>,
}

/// Enumeration of the different badges used in the BILLA Online Shop.
///
/// These badges are used to indicate some quality of a product, that is either
/// some private or public sign of quality (e.g. FairTrade, Rainforest, DOP),
/// a hint for a specific diet (e.g. vegan, gluten-free), a hint for how the
/// product should be stored (e.g. deep-frozen) or something more ambigous
/// (e.g. new product, product from Austria).
#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct BillaPriceInfo {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BillaResultFacet {
    key: String,
    label: String,
    is_multi_select: bool,
    terms: Option<Vec<BillaFacetTerm>>,
    comparison_terms: Option<Vec<BillaFacetTerm>>,
    range: Option<BillaFacetRange>,
}

#[derive(Debug, Deserialize)]
struct BillaFacetTerm {
    key: String,
    label: String,
    term: String,
    count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BillaFacetRange {
    key: String,
    label: String,
    count: usize,
    label_symbol: String,
    min: f64,
    max: f64,
}
