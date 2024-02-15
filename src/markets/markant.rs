use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkantAttributes {
    data: Option<Vec<MarkantData>>,
    metadata: Option<Vec<MarkantMetadata>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MarkantData {
    //    additional_food_info: Option<AdditionalFoodInfo>,
    //    additional_information: Option<AdditionalInfo>,
    //    additive: Option<AdditiveInfo>,
    //    alcohol_information: Option<AlcoholInfo>,
    //    allergen: Option<AllergenInfo>,
    //    allergen_types: Option<Vec<String>>,
    //    animal_feed_additive_statement: Option<AnimalFeedAdditiveInfo>,
    //    animal_feed_composition_statement: Option<AnimalFeedCompositionInfo>,
    //    animal_feed_constituents_statement: Option<AnimalFeedConsituentsInfo>,
    //    animal_feeding: Option<AnimalFeeding>,
    //    animal_feeding_instructions: Option<AnimalFeedingInstructions>,
    //    battery: Option<BatteryInfo>,
    //    cheese: Option<CheeseInfo>,
    //    consumer: Option<ConsumerInfo>,
    //    contact_information: Option<ContactInfo>,
    //    description: Option<Description>,
    //    egg: Option<EggInfo>,
    //    fish: Option<FishInfo>,
    //    ghs_information: Option<GhsInfo>,
    gtin: Option<String>,
    //    identification: Option<Identification>,
    //    ingredients: Option<Ingredrients>,
    last_update_timestamp: Option<String>,
    //    lifespan: Option<LifespanInfo>,
    measurement: Option<MeasurementInfo>,
    //    meat: Option<MeatInfo>,
    //    mpreis_trade_item: Option<TradeItem>,
    //    near_food: Option<NearFoodInfo>,
    //    nutrient: Option<NutrientInfo>,
    //    nutrition: Option<NutritionInfo>,
    //    organic_information: Option<OrganicInfo>,
    //    origin: Option<OriginInfo>,
    //    packaging_information: Option<PackagingInfo>,
    packaging_labels_accreditation: Option<Vec<String>>,
    //    packaging_marking: Option<PackagingMarking>,
    preparation: Option<PreparationInfo>,
    temperature_information: Option<TemperatureInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MeasurementInfo {
    drained_weight: Option<f64>,
    drained_weight_uom: Option<String>,
    gross_weight: Option<f64>,
    gross_weight_uom: Option<String>,
    measurement_net_content: Option<HashMap<u16, MeasurementNetContent>>,
    net_content_statement: Option<HashMap<u16, NetContentInfo>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MeasurementNetContent {
    net_content: Option<f32>,
    net_content_uom: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NetContentInfo {
    net_content_statement: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PreparationInfo {
    preparation_instruction: Option<Vec<PreparationInstruction>>,
    preparation_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PreparationInstruction {
    preparation_instruct: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TemperatureInfo {
    storage_handling_max: Option<f32>,
    storage_handling_max_uom: Option<String>,
    storage_handling_min: Option<f32>,
    storage_handling_min_uom: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MarkantMetadata {
    markant_data: Option<bool>,
}
