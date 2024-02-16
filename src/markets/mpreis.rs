use crate::markets::algolia::AlgoliaBrowseResponse;
use crate::markets::markant::MarkantAttributes;
use serde::{Deserialize, Serialize};

pub type MpreisBrowseResponse = AlgoliaBrowseResponse<MpreisProduct>;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MpreisProduct {
    /// Whether the product is in stock.
    available: bool,

    /// The list of categories in reverse hierarchical order.
    ///
    /// This list should always contain the `"ProductRoot"` as the last element.
    categories: Option<Vec<String>>,

    /// Whether the category is missing.
    categories_missing: bool,

    /// The recursive categories structure with more category metadata.
    category: Option<MpreisCategory>,
    #[serde(rename = "category_ids")]
    category_ids: Option<Vec<String>>,
    #[serde(rename = "category_levels")]
    category_levels: Option<MpreisCategoryLevels>,

    /// The internal identifier of the product.
    code: String,

    /// The list of descriptions for the product.
    ///
    /// This field usually contains only one item, which is often the same as
    /// the name. It should not be used or relied on. For a more concise
    /// description use one in the [MpreisCustomAttributes].
    description: Vec<String>,
    distribution_channel: Vec<String>,
    fees: Option<Vec<MpreisFeeInfo>>,

    /// The absolute URL to a JPEG image of the product.
    ///
    /// If this element is not present, the fallback
    /// `"https://www.mpreis.at/assets/noImage_detail-5sxJ3bpG.png"` should be
    /// used.
    image: Option<String>,

    /// Whether the `image` field is defined.
    ///
    /// This field is unreliable as sometimes it is not set even though the
    /// `image` field is set.
    image_missing: Option<bool>,

    /// The localizations for the product description.
    localized_description: Option<MpreisLocalizedString>,

    /// The localizations for the product name.
    localized_name: Option<MpreisLocalizedString>,

    /// A lot of metadata and attribute mixins, describing the product more clearly.
    mixins: Option<MpreisMixin>,

    /// The list of names for the product.
    ///
    /// This field usually contains only one item and the length of the strings
    /// seems to be capped at 40 characters. It is quite unclear when the brand
    /// name is in the name or not. The brand name is also sometimes abbreviated.
    /// It should not be used or relied on. For a more concise name use one in
    /// the [MpreisAttributes].
    name: Vec<String>,

    /// Object identifier for the product.
    ///
    /// The value always starts with `urn:yaas:saasag:caasproduct:product:mpreis;`
    /// followed by the `code` of the product.
    #[serde(rename = "objectID")]
    object_id: String,

    /// The list of prices for the product.
    ///
    /// The field does only contain one element.
    prices: Vec<MpreisPriceInfo>,

    /// Internal tags to describe type and state.
    ///
    /// This field seems to be either `product` or `published`.
    #[serde(rename = "_tags")]
    tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisCategory {
    code: Option<String>,
    id: Option<String>,
    localized_name: (),
    localized_slug: (),
    name: Option<String>,
    // parent: MpreisCategory, TODO Recursive data structure
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisCategoryLevels {
    level0: Option<Vec<String>>,
    level1: Option<Vec<String>>,
    level2: Option<Vec<String>>,
    level3: Option<Vec<String>>,
    level4: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisFeeInfo {
    fee_amount: Option<f64>,
    fee_currency: Option<String>,
    fee_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MpreisLocalizedString {
    en: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisMixin {
    markant_attributes: Option<MarkantAttributes>,
    mpreis_attributes: MpreisAttributes,
    product_custom_attributes: MpreisCustomAttributes,
    wine_attributes: Option<MpreisWineAttributes>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisAttributes {
    acn: Vec<MpreisACN>,
    #[serde(rename = "available_post")]
    available_post: Option<bool>,
    ean: Option<String>,
    eans: Option<Vec<String>>,
    fee: Option<f64>,
    fee_id: Option<String>,
    fee_tax_code: Option<String>,
    gtins: Option<Vec<MpreisGTIN>>,
    image_gtins: Option<Vec<MpreisGTIN>>,
    #[serde(rename = "mhd_min")]
    mhd_min: u32,
    popularity: Option<f32>,
    product_name: String,
    properties: Option<Vec<String>>,
    supplier: String,
    unit_logic: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisACN {
    acn: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisGTIN {
    content: Option<u32>,
    gln: Option<String>,
    gtin: Option<String>,
    kz: Option<String>,
    take_from: Option<String>,
    tm: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisCustomAttributes {
    default_order_quantity: u32,
    in_stock: Option<bool>,
    labels: Option<Vec<String>>,
    long_description: Option<String>,
    max_order_quantity: f32,
    min_order_quantity: f32,
    order_unit: String,
    origin_country: String,
    packaging_description: String,
    packaging_unit: String,
    pricing_measure: MpreisPricingMeasure,
    product_state: String,
    tax_class: String,
    unit_pricing_base_measure: MpreisPricingMeasure,
    unit_pricing_measure: MpreisPricingMeasure,
    weight_dependent: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisPricingMeasure {
    unit_code: String,
    value: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisWineAttributes {
    alcohol_concentration: Option<String>,
    alcohol_density: Option<String>,
    award: Option<String>,
    barrel_aged: Option<String>,
    bottle_cap: Option<String>,
    characteristic: Option<String>,
    colour: Option<String>,
    cuvee_components: Option<String>,
    food_pairing: Option<String>,
    food_recommendation: Option<String>,
    grape_variety: Option<String>,
    quality_classification: Option<String>,
    serving_temperature: Option<String>,
    sulfite_identification: Option<String>,
    sustainability_certificate: Option<String>,
    sweetness: Option<String>,
    wine_region: Option<String>,
    wine_style: Option<String>,
    winery: Option<String>,
    year: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisPriceInfo {
    /// The base price of the product, which is the price per unit.
    base_price: MpreisPrice,

    /// The ISO 4217 currency code the price is valued in.
    currency: String,

    /// The custom attributes of the price information.
    custom_attributes: MpreisPriceAttributes,

    /// The effective price the product is priced at now.
    effective_amount: f64,

    /// The measurement unit of the original and effective amount.
    measurement_unit: MpreisMeasurementUnit,

    /// The list of BOGO discounts that are available on this product's price.
    natural_discounts: Option<Vec<MpreisBogoDiscount>>,

    /// The standard price the product usually is priced at.
    original_amount: f64,

    /// The price to display at the web page.
    ///
    /// This value seems to be sometimes a copy of the prices in [MpreisPriceInfo]
    /// but sometimes deviates from that, especially for products that are
    /// currently unavailable. It should **not be used**.
    presentation_price: MpreisPrice,

    /// The 24 (alphanumeric) character string id.
    price_id: String,

    /// The code of the site for the price (which is `8450` in case of MPreis).
    site_code: String,

    /// The three (alphanumeric) character type code.
    ///
    /// This value seems to be enumerated with the values:
    /// `"V1NO"`, `"V1HO"`, `"V3BP"`, `"V1BE"`, `"V4AB"`, `"V3SO"`, `"V1SS"`
    #[serde(rename = "type")]
    price_type: Option<String>,

    /// The information about the wholesale discount, if one is applicable.
    wholesale: Option<MpreisWholesale>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisPrice {
    /// The effective price the product is priced at now.
    effective_amount: f64,

    /// The measurement unit of the original and effective amount.
    measurement_unit: MpreisMeasurementUnit,

    /// The standard price the product usually is priced at.
    original_amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisPriceAttributes {
    /// THe description of the promotion, which always equals the name.
    promotion_description: Option<String>,

    /// The 12 character string identifier that always starts with `"MPR-"`.
    promotion_id: Option<String>,

    /// The name of the promotion, which always equals the description.
    ///
    /// This value seems to be capped at 25 characters, similar to the receipts
    /// you receive at the shop.
    promotion_name: Option<String>,

    /// The priority of the promotion.
    ///
    /// It is unclear what it means, but it is either `100.0` or `102.0`.
    promotion_priority: Option<String>,
    promotion_reference: Option<String>,

    /// The type of the promotion for the product price.
    promotion_type: MpreisPromotionType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum MpreisPromotionType {
    /// Wholesole discount promotion.
    ///
    /// This promotion type is characterized with a promotion that is valid if
    /// the customer buys a specific minimum amount of the product.
    Ab,

    /// Generic discount promotion.
    ///
    /// This promotion type represents any discount which lowers the price of
    /// the product without any special conditions.
    Dis,

    /// BOGO (buy-one-get-one-free) discount promotion.
    ///
    /// This promotion type is not limited to "1+1", but can consist of an
    /// arbitrary combination which needs to be calculated with [MpreisBogoDiscount].
    Pak,

    /// No discount promotion.
    ///
    /// This promotion type is just a fallback if the product doesn't have any
    /// discounts and is at its regular price.
    RegularPrice,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisMeasurementUnit {
    quantity: f64,
    unit_code: String,
}

/// This is the representation of a BOGO discount from the Mpreis API.
///
/// A BOGO discount is a buy-one-get-one-free discount, which is presented as
/// a badge with something like "1+1 gratis", "2+1 gratis" to suggest the
/// customer that they get something free from buying this now.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisBogoDiscount {
    /// The effective date range while this discount lasts.
    date_range: Option<MpreisDateRange>,

    /// The amount of items that are represented as "free".
    ///
    /// For example, if the badge says "3+1 gratis", then this value is 1.
    discount_quantity: Option<f32>,

    /// The whole amount of items that need to be bought.
    ///
    /// For example, if the badge says "3+1 gratis", then this value is 4.
    for_quantity: Option<f32>,

    /// The 12 character string identifier that always starts with `"MPR-"`.
    promotion_id: Option<String>,
    reference: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisDateRange {
    end_date: Option<String>,
    start_date: Option<String>,
}

/// This is the representation for a wholesale from the Mpreis API.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisWholesale {
    /// The minimum amount that needs to be bought so that the discount is applied.
    min_quantity: f32,
}
