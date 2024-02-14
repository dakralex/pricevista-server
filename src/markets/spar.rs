use crate::markets::factfinder;
use serde::{Deserialize, Serialize};

pub type SparSearchResponse = factfinder::SearchResponse<SparProduct>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SparProduct {
    #[serde(flatten)]
    product: ProductInfo,
    #[serde(flatten)]
    category: CategoryInfo,
    #[serde(flatten)]
    packaging: PackagingInfo,
    #[serde(flatten)]
    price: PriceInfo,
    #[serde(flatten)]
    nutrition: NutritionInfo,
    #[serde(flatten)]
    alcohol: AlcoholInfo,
    #[serde(flatten)]
    misc: MiscellaneousInfo,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ProductInfo {
    brand: Option<Vec<String>>,
    code_internal: String,
    #[serde(rename = "land")]
    country: Option<String>,
    created_at: String,
    description: Option<String>,
    ecr_brand: Option<String>,
    image_url: String,
    marketing_text: Option<String>,
    name: String,
    product_number: String,
    short_description: Option<String>,
    short_description_2: Option<String>,
    short_description_3: Option<String>,
    title: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct CategoryInfo {
    categories: Option<Vec<String>>,
    category_facet: Option<String>,
    category_id: Option<Vec<String>>,
    category_names: Option<String>,
    category_path: Option<Vec<Vec<String>>>,
    ecr_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct PackagingInfo {
    additional_quantity: Option<String>,
    approx_weight_product: Option<String>,
    item_type: String,
    packing_type: Option<String>,
    quantity_selector: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct PriceInfo {
    actual_price: Option<f64>,
    best_price: f64,
    is_on_promotion: String,
    price: f64,
    regular_price: f64,
    price_per_unit: String,
    promotion_most_likely_text: Option<String>,
    promotion_text: Option<String>,
    sales_unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct NutritionInfo {
    badge_icon: Option<String>,
    badge_names: Option<String>,
    badge_short_name: Option<String>,
    #[serde(rename = "biolevel")]
    bio_level: Option<String>,
    customer_general_info_name: Vec<String>,
    customer_info: Option<Vec<String>>,
    fabric_base: Option<String>,
    fat_content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct AlcoholInfo {
    area: Option<String>,
    alcohol_age: Option<String>,
    region: Option<String>,
    taste: Option<String>,
    year: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct MiscellaneousInfo {
    color_filter: Option<Vec<String>>,
    #[serde(rename = "finalsalescheckok")]
    final_sales_check_ok: Option<String>,
    is_new: String,
    is_restaurant_product: String,
    pos_purchasable: Option<Vec<String>>,
    pos_visible: Option<Vec<String>>,
    recipe: Option<String>,
    #[serde(rename = "relevantfororder")]
    relevant_for_order: String,
    shop_source: String,
    sort: Option<String>,
    stock_status: String,
    target_group: Option<String>,
}
