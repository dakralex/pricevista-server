use crate::markets::factfinder;
use serde::{Deserialize, Serialize};

pub type SparSearchResponse = factfinder::SearchResponse<SparProduct>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SparProduct {
    actual_price: Option<f64>,
    additional_quantity: Option<String>,
    alcohol_age: Option<String>,
    approx_weight_product: Option<String>,
    area: Option<String>,
    badge_icon: Option<String>,
    badge_names: Option<String>,
    badge_short_name: Option<String>,
    best_price: f64,
    #[serde(rename = "biolevel")]
    bio_level: Option<String>,
    brand: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    category_facet: Option<String>,
    category_id: Option<Vec<String>>,
    category_names: Option<String>,
    category_path: Option<Vec<Vec<String>>>,
    code_internal: String,
    color_filter: Option<Vec<String>>,
    created_at: String,
    customer_general_info_name: Vec<String>,
    customer_info: Option<Vec<String>>,
    description: Option<String>,
    ecr_brand: Option<String>,
    ecr_type: Option<String>,
    fabric_base: Option<String>,
    fat_content: Option<String>,
    #[serde(rename = "finalsalescheckok")]
    final_sales_check_ok: Option<String>,
    image_url: String,
    is_new: String,
    is_on_promotion: String,
    is_restaurant_product: String,
    item_type: String,
    #[serde(rename = "land")]
    country: Option<String>,
    marketing_text: Option<String>,
    name: String,
    packing_type: Option<String>,
    pos_purchasable: Option<Vec<String>>,
    pos_visible: Option<Vec<String>>,
    price: f64,
    price_per_unit: String,
    product_number: String,
    promotion_most_likely_text: Option<String>,
    promotion_text: Option<String>,
    quantity_selector: Option<String>,
    recipe: Option<String>,
    region: Option<String>,
    regular_price: f64,
    #[serde(rename = "relevantfororder")]
    relevant_for_order: String,
    sales_unit: String,
    shop_source: String,
    short_description: Option<String>,
    short_description_2: Option<String>,
    short_description_3: Option<String>,
    sort: Option<String>,
    stock_status: String,
    target_group: Option<String>,
    taste: Option<String>,
    title: String,
    url: String,
    year: Option<String>,
}
