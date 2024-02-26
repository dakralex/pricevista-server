use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkantAttributes {
    data: Option<Vec<MarkantData>>,
    // metadata: Option<Vec<MarkantMetadata>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MarkantData {
    additional_food_info: Option<AdditionalFoodInfo>,
    additional_information: Option<AdditionalInformation>,
    additive: Option<Additive>,
    alcohol_information: Option<AlcoholInformation>,
    allergen: Option<Allergen>,
    // allergen_types: Option<BTreeMap<String, String>>,
    animal_feed_additive_statement: Option<BTreeMap<String, AnimalFeedAdditiveStatement>>,
    animal_feed_composition_statement: Option<BTreeMap<String, AnimalFeedCompositionStatement>>,
    animal_feed_constituents_statement: Option<BTreeMap<String, AnimalFeedConstituentsStatement>>,
    animal_feeding: Option<BTreeMap<String, AnimalFeeding>>,
    animal_feeding_instructions: Option<BTreeMap<String, AnimalFeedingInstruction>>,
    battery: Option<Battery>,
    cheese: Option<Cheese>,
    consumer: Option<Consumer>,
    contact_information: Option<ContactInformation>,
    description: Option<Description>,
    egg: Option<Egg>,
    fish: Option<Fish>,
    ghs_information: Option<GhsInformation>,
    gtin: Option<String>,
    identification: Option<Identification>,
    ingredients: Option<Ingredients>,
    last_update_timestamp: Option<String>,
    lifespan: Option<Lifespan>,
    measurement: Option<MeasurementInfo>,
    meat: Option<Meat>,
    mpreis_trade_item: Option<MpreisTradeItem>,
    near_food: Option<NearFood>,
    nutrient: Option<Nutrient>,
    nutrition: Option<Nutrition>,
    organic_information: Option<OrganicInformation>,
    origin: Option<Origin>,
    packaging_information: Option<PackagingInformation>,
    // packaging_labels_accreditation: Option<BTreeMap<String, String>>,
    packaging_marking: Option<PackagingMarking>,
    preparation: Option<Preparation>,
    temperature_information: Option<TemperatureInformation>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdditionalFoodInfo {
    additive_label_information: Option<BTreeMap<String, AdditiveLabelInformation>>,
    health_claim: Option<BTreeMap<String, HealthClaim>>,
    package_disclaimer: Option<PackageDisclaimer>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdditiveLabelInformation {
    additive_label_information: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthClaim {
    health_claim_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PackageDisclaimer {
    language: Option<String>,
    package_disclaimer: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdditionalInformation {
    aise_safe_use_icon: Option<BTreeMap<String, AiseSafeUseIcon>>,
    delight_region_austria: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AiseSafeUseIcon(Option<String>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Additive {
    additive_declaration: Option<BTreeMap<String, AdditiveDeclaration>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdditiveDeclaration {
    level_of_containment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AlcoholInformation {
    alcohol_grape_variety: Option<BTreeMap<String, AlcoholGrapeVariety>>,
    alcoholic_beverage_subregion: Option<String>,
    colour_of_wine_alcohol: Option<String>,
    percentage_of_alcohol_by_volume: Option<Number>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlcoholGrapeVariety(Option<String>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Allergen {
    allergen_statement: Option<AllergenStatement>,
    allergen_type: Option<AllergenType>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AllergenStatement {
    allergen_statement: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AllergenType {
    allergen_type: Option<String>,
    level_of_containment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AnimalFeedAdditiveStatement {
    feed_additive_statement: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AnimalFeedCompositionStatement {
    feed_composition_statement: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AnimalFeedConstituentsStatement {
    feed_constituents_statement: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AnimalFeeding {
    feed_life_stage: Option<BTreeMap<String, FeedLifeStage>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct FeedLifeStage {
    feed_lifestage: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AnimalFeedingInstruction {
    feeding_instruction: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Battery {
    are_batteries_included: Option<String>,
    are_batteries_required: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Cheese {
    fat_percentage_in_dry_matter: Option<Number>,
    is_rind_edible: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Consumer {
    consumer_storage: Option<BTreeMap<String, ConsumerStorage>>,
    consumer_usage: Option<BTreeMap<String, ConsumerUsage>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConsumerStorage {
    consumer_storage_instructions: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConsumerUsage {
    consumer_usage_instructions: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContactInformation {
    brand_owner_name: Option<String>,
    contact_address: Option<BTreeMap<String, ContactAddress>>,
    contact_name: Option<String>,
    manufacturer_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ContactAddress(Option<String>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Description {
    brand_name: Option<String>,
    description_short: Option<BTreeMap<String, DescriptionShort>>,
    feature_benefit: Option<BTreeMap<String, FeatureBenefit>>,
    marketing_message: Option<BTreeMap<String, MarketingMessage>>,
    regulated_product_name: Option<BTreeMap<String, RegulatedProductName>>,
    sub_brand: Option<String>,
    trade_item_description: Option<BTreeMap<String, TradeItemDescription>>,
    variant_description: Option<BTreeMap<String, VariantDescription>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Egg {
    quality_class_for_eggs: Option<String>,
    weight_class_for_eggs: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Fish {
    species_statistics_purposes: Option<String>,
    species_statistics_purposes_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GhsInformation {
    hazard_statement: Option<BTreeMap<String, HazardStatement>>,
    precautionary_statement: Option<BTreeMap<String, PrecautionaryStatement>>,
    regulatory_permit_ident: Option<String>,
    signal_words: Option<String>,
    // symbol_description: Option<Vec<SymbolDescription>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct HazardStatement {
    hazard_statement: Option<String>,
    hazard_statement_description: Option<BTreeMap<String, HazardStatementDescription>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct HazardStatementDescription {
    hazard_statement_description: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PrecautionaryStatement {
    precautionary_statement: Option<String>,
    precautionary_statement_description:
        Option<BTreeMap<String, PrecautionaryStatementDescription>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PrecautionaryStatementDescription {
    language: Option<String>,
    precautionary_statement_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SymbolDescription(Option<String>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Identification {
    biocide_regulation_code: Option<String>,
    cancelled_date: Option<String>,
    discontinued_date: Option<String>,
    gpc_attribute_value: Option<BTreeMap<String, GpcAttributeValue>>,
    last_change_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GpcAttributeValue {
    gpc_attribute_type: Option<String>,
    gpc_attribute_value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Ingredients {
    ingredient: Option<BTreeMap<String, Ingredient>>,
    ingredient_statement: Option<BTreeMap<String, IngredientStatement>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Ingredient {
    content_percentage: Option<Number>,
    genetically_modified: Option<String>,
    ingredient_name: Option<IngredientName>,
    sequence: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct IngredientName {
    ingredient_name: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct IngredientStatement {
    ingredient_statement: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DescriptionShort {
    description_short: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct FeatureBenefit {
    feature_benefit: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MarketingMessage {
    marketing_message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RegulatedProductName {
    language: Option<String>,
    regulated_product_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TradeItemDescription {
    language: Option<String>,
    trade_item_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct VariantDescription {
    variant_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Lifespan {
    safe_to_use_after_opening: Option<Number>,
    safe_to_use_after_opening_uom: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MeasurementInfo {
    drained_weight: Option<Number>,
    drained_weight_uom: Option<String>,
    gross_weight: Option<Number>,
    gross_weight_uom: Option<String>,
    measurement_net_content: Option<BTreeMap<String, MeasurementNetContent>>,
    net_content_statement: Option<BTreeMap<String, NetContentStatement>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MeasurementNetContent {
    net_content: Option<Number>,
    net_content_uom: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NetContentStatement {
    net_content_statement: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Meat {
    meat_place_of_birth: Option<BTreeMap<String, MeatDetail>>,
    meat_place_of_rearing: Option<BTreeMap<String, MeatDetail>>,
    meat_place_of_slaughter: Option<BTreeMap<String, MeatDetail>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MeatDetail {
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MpreisTradeItem {
    additional_descriptions: Option<BTreeMap<String, AdditionalDescription>>,
    additional_party_identifications: Option<BTreeMap<String, AdditionalPartyIdentification>>,
    additive_classes: Option<BTreeMap<String, AdditiveClass>>,
    additive_enumbers: Option<BTreeMap<String, AdditiveEnumber>>,
    alcoholic_beverage_subregion: Option<String>,
    average_life: Option<String>,
    colour_of_wine_alcohol: Option<String>,
    country_of_last_processing: Option<String>,
    dangerous_substance_names: Option<BTreeMap<String, DangerousSubstanceName>>,
    degree_of_original_wort: Option<String>,
    fat_in_milk_content: Option<String>,
    grape_varieties: Option<BTreeMap<String, AlcoholGrapeVariety>>,
    is_homogenised: Option<String>,
    net_weight: Option<String>,
    net_weight_uom: Option<String>,
    non_food_compulsories: Option<BTreeMap<String, NonFoodCompulsory>>,
    order_quantity_minimum: Option<String>,
    preservation_techniques: Option<BTreeMap<String, PreservationTechnique>>,
    production_facilities: Option<BTreeMap<String, ProductionFacility>>,
    ripening_time_period: Option<String>,
    ripening_time_period_uom: Option<String>,
    safety_data_sheet_number: Option<String>,
    sugar_grams_per_litre: Option<String>,
    sweetness_level_of_alcoholic_beverage: Option<String>,
    temperature_condition: Option<String>,
    type_of_bottle_closure: Option<String>,
    vintner: Option<String>,
    water_hazard_code: Option<String>,
    water_solubility: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdditionalDescription {
    additional_description: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdditionalPartyIdentification {
    additional_party_identification: Option<String>,
    additional_party_identification_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdditiveClass {
    class_code: Option<String>,
    level_of_containment_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdditiveEnumber {
    enumber_code: Option<String>,
    level_of_containment_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DangerousSubstanceName(Option<String>);

#[derive(Debug, Deserialize, Serialize)]
struct NonFoodCompulsory(Option<String>);

#[derive(Debug, Deserialize, Serialize)]
struct PreservationTechnique(Option<String>);

#[derive(Debug, Deserialize, Serialize)]
struct ProductionFacility(Option<String>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NearFood {
    nappy_diaper_size: Option<String>,
    number_of_plys: Option<String>,
    sun_protection_factor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Nutrient {
    nutrient_header: Option<BTreeMap<String, NutrientHeader>>,
    nutrient_information: Option<NutrientInformation>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NutrientHeader {
    daily_value_intake_reference: Option<BTreeMap<String, DailyValueIntakeReference>>,
    energy_kcal: Option<Number>,
    energy_kj: Option<Number>,
    energy_percentage_of_daily_intake: Option<Number>,
    nutrient_basis_quantity: Option<Number>,
    nutrient_basis_quantity_uom: Option<String>,
    nutrient_content: Option<BTreeMap<String, NutrientHeaderEntry>>,
    nutrient_other: Option<BTreeMap<String, NutrientHeaderEntry>>,
    nutrient_vitamin: Option<BTreeMap<String, NutrientHeaderEntry>>,
    preparation_state: Option<String>,
    serving_size: Option<Number>,
    serving_size_uom: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DailyValueIntakeReference {
    daily_value_intake_reference: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NutrientHeaderEntry {
    nutrient_type: Option<String>,
    percentage_of_daily_value_intake: Option<Number>,
    quantity_contained: Option<Number>,
    quantity_contained_uom: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NutrientInformation {
    servings_per_package: Option<Number>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Nutrition {
    nutritional_claim: Option<BTreeMap<String, NutritionalClaim>>,
    nutritional_claim_detail: Option<BTreeMap<String, NutritionalClaimDetail>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NutritionalClaim {
    nutritional_claim: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NutritionalClaimDetail {
    claim_nutrient_element: Option<String>,
    claim_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct OrganicInformation {
    certification_body: Option<String>,
    country_of_origin_farming: Option<String>,
    genetically_modified: Option<String>,
    organic_growing_method: Option<BTreeMap<String, OrganicGrowingMethod>>,
    origin: Option<String>,
    place_of_farming: Option<String>,
    quality_symbol: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OrganicGrowingMethod(Option<String>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PackagingInformation {
    packaging: Option<BTreeMap<String, Packaging>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Packaging {
    packaging_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PackagingMarking {
    packaging_label_accreditation: Option<BTreeMap<String, PackagingLabelAccreditation>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PackagingLabelAccreditation(Option<String>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Origin {
    country_of_origin: Option<String>,
    place_of_provenance: Option<BTreeMap<String, PlaceOfProvenance>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PlaceOfProvenance {
    place_of_provenance: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Preparation {
    preparation_instruction: Option<BTreeMap<String, PreparationInstruction>>,
    preparation_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PreparationInstruction {
    preparation_instruction: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TemperatureInformation {
    storage_handling_max: Option<Number>,
    storage_handling_max_uom: Option<String>,
    storage_handling_min: Option<Number>,
    storage_handling_min_uom: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MarkantMetadata {
    markant_data: Option<bool>,
}
