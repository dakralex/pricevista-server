use crate::markets::algolia::AlgoliaBrowseResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type MpreisBrowseResponse = AlgoliaBrowseResponse<MpreisProduct>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MpreisProduct {
    /// Whether the product is available.
    available: bool,
    categories: Option<Vec<String>>,
    categories_missing: bool,
    category: Option<MpreisCategory>,
    #[serde(rename = "category_ids")]
    category_ids: Option<Vec<String>>,
    #[serde(rename = "category_levels")]
    category_levels: Option<MpreisCategoryLevels>,

    /// Internal identifier of the product.
    code: String,

    /// Array of descriptions for the product.
    description: Vec<String>,
    distribution_channel: Vec<String>,
    fees: Option<Vec<MpreisFeeInfo>>,

    /// Absolute URL to a JPEG image of the product.
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
    mixins: MpreisMixin,

    /// Array of (inconsistent) names for the product.
    ///
    /// This field's elements seem to be capped at 40 characters and are quite
    /// inconsistent as they sometimes contain the brand name and sometimes
    /// not.
    name: Vec<String>,

    /// Object identifier for the product.
    ///
    /// The value always starts with `urn:yaas:saasag:caasproduct:product:mpreis;`
    /// followed by the `code` of the product.
    #[serde(rename = "objectID")]
    object_id: String,
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
    markant_attributes: Option<MpreisMarkantAttributes>,
    mpreis_attributes: MpreisAttributes,
    product_custom_attributes: MpreisCustomAttributes,
    wine_attributes: Option<MpreisWineAttributes>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisMarkantAttributes {
    data: Option<Vec<MpreisMarkantData>>,
    //    metadata: Option<Vec<MpreisMarkantMetadata>>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisMarkantData {
    //    additional_food_info: Option<MpreisAdditionalFoodInfo>,
    //    additional_information: Option<MpreisAdditionalInfo>,
    //    additive: Option<MpreisAdditiveInfo>,
    //    alcohol_information: Option<MpreisAlcoholInfo>,
    //    allergen: Option<MpreisAllergenInfo>,
    //    allergen_types: Option<Vec<String>>,
    //    animal_feed_additive_statement: Option<MpreisAnimalFeedAdditiveInfo>,
    //    animal_feed_composition_statement: Option<MpreisAnimalFeedCompositionInfo>,
    //    animal_feed_constituents_statement: Option<MpreisAnimalFeedConsituentsInfo>,
    //    animal_feeding: Option<MpreisAnimalFeeding>,
    //    animal_feeding_instructions: Option<MpreisAnimalFeedingInstructions>,
    //    battery: Option<MpreisBatteryInfo>,
    //    cheese: Option<MpreisCheeseInfo>,
    //    consumer: Option<MpreisConsumerInfo>,
    //    contact_information: Option<MpreisContactInfo>,
    //    description: Option<MpreisDescription>,
    //    egg: Option<MpreisEggInfo>,
    //    fish: Option<MpreisFishInfo>,
    //    ghs_information: Option<MpreisGhsInfo>,
    gtin: Option<String>,
    //    identification: Option<MpreisIdentification>,
    //    ingredients: Option<MpreisIngredrients>,
    last_update_timestamp: Option<String>,
    //    lifespan: Option<MpreisLifespanInfo>,
    measurement: Option<MpreisMeasurementInfo>,
    //    meat: Option<MpreisMeatInfo>,
    //    mpreis_trade_item: Option<MpreisTradeItem>,
    //    near_food: Option<MpreisNearFoodInfo>,
    //    nutrient: Option<MpreisNutrientInfo>,
    //    nutrition: Option<MpreisNutritionInfo>,
    //    organic_information: Option<MpreisOrganicInfo>,
    //    origin: Option<MpreisOriginInfo>,
    //    packaging_information: Option<MpreisPackagingInfo>,
    packaging_labels_accreditation: Option<Vec<String>>,
    //    packaging_marking: Option<MpreisPackagingMarking>,
    preparation: Option<MpreisPreparationInfo>,
    temperature_information: Option<MpreisTemperatureInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisMeasurementInfo {
    drained_weight: Option<f64>,
    drained_weight_uom: Option<String>,
    gross_weight: Option<f64>,
    gross_weight_uom: Option<String>,
    measurement_net_content: Option<HashMap<u16, MpreisMeasurementNetContent>>,
    net_content_statement: Option<HashMap<u16, MpreisNetContentInfo>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisMeasurementNetContent {
    net_content: Option<f32>,
    net_content_uom: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisNetContentInfo {
    net_content_statement: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisPreparationInfo {
    preparation_instruction: Option<Vec<MpreisPreparationInstruction>>,
    preparation_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisPreparationInstruction {
    preparation_instruct: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisTemperatureInfo {
    storage_handling_max: Option<f32>,
    storage_handling_max_uom: Option<String>,
    storage_handling_min: Option<f32>,
    storage_handling_min_uom: Option<String>,
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
    base_price: MpreisPrice,
    currency: String,
    custom_attributes: MpreisPriceAttributes,
    effective_amount: f64,
    measurement_unit: MpreisMeasurementUnit,
    natural_discounts: Option<Vec<MpreisNaturalDiscount>>,
    original_amount: f64,
    presentation_price: MpreisPrice,
    price_id: String,
    site_code: String,
    #[serde(rename = "type")]
    price_type: Option<String>,
    wholesale: Option<MpreisWholesale>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisPrice {
    effective_amount: f64,
    measurement_unit: MpreisMeasurementUnit,
    original_amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisPriceAttributes {
    promotion_description: Option<String>,
    promotion_id: Option<String>,
    promotion_name: Option<String>,
    promotion_priority: Option<String>,
    promotion_reference: Option<String>,
    promotion_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisMeasurementUnit {
    quantity: f64,
    unit_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisNaturalDiscount {
    date_range: Option<MpreisDateRange>,
    discount_quantity: Option<f32>,
    for_quantity: Option<f32>,
    promotion_id: Option<String>,
    reference: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisDateRange {
    end_date: Option<String>,
    start_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisWholesale {
    min_quantity: f32,
}
