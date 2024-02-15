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
struct MarkantMetadata {
    markant_data: Option<bool>,
}
