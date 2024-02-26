use crate::providers::roksh;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Number;

pub async fn fetch_hofer() {
    let client = Client::new();

    let bearer = roksh::get_session_bearer(&client, "hofer")
        .await
        .unwrap()
        .unwrap();
    let categories = roksh::get_full_category_list(&client, &bearer)
        .await
        .unwrap();
    let products = roksh::get_category_product_list(&client, &bearer)
        .await
        .unwrap();

    println!("{}", products)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoferProduct {
    available: bool,
    #[serde(rename = "basketItemID")]
    basket_item_id: Option<String>,
    brand: Option<String>,
    category: HoferCategory,
    #[serde(rename = "categoryID")]
    category_id: u32,
    #[serde(rename = "categorySEOName")]
    category_seo_name: String,
    deposit_fee: Option<f64>,
    description: String,
    display_unit: String,
    gallery_image_url_list: Option<String>,
    icons: Vec<HoferIcon>,
    is_bulk: bool,
    #[serde(rename = "isDiscountPD")]
    is_discount_pd: Option<String>,
    #[serde(rename = "isDIscountPPDD")]
    is_discount_ppdd: Option<String>,
    is_offer: bool,
    is_one_product_for_free: bool,
    is_product_roksh_discounted: bool,
    manufacturer: Option<String>,
    max_weight_step: u32,
    #[serde(rename = "mediaUrlL")]
    media_url_large: String,
    #[serde(rename = "mediaUrlM")]
    media_url_medium: String,
    #[serde(rename = "mediaUrlS")]
    media_url_small: String,
    min_price: f64,
    min_unit_price: f64,
    min_weight_step: f64,
    offer_valid_to: String,
    original_price_if_offer: Option<String>,
    original_unit_price_if_offer: Option<String>,
    price: f64,
    price_unit_type: Option<String>,
    product_details: HoferProductDetails,
    #[serde(rename = "productID")]
    product_id: u32,
    product_media_list: Vec<HoferProductMediaItem>,
    product_name: String,
    product_provider: Vec<HoferProductProvider>,
    #[serde(rename = "productProviderID")]
    product_provider_id: u32,
    provider_available: bool,
    provider_deposit_product_dto_list: Vec<String>,
    provider_order_number: u32,
    roksh_discount_basket_value: Option<u32>,
    roksh_discount_level: Option<u32>,
    roksh_discount_price: Option<f64>,
    #[serde(rename = "sEOMetaDescription")]
    seo_meta_description: Option<String>,
    #[serde(rename = "sEOMetaKeyWords")]
    seo_meta_keywords: Option<String>,
    #[serde(rename = "sEOName")]
    seo_name: Option<String>,
    #[serde(rename = "sEOTitle")]
    seo_title: Option<String>,
    secondary_unit: Option<String>,
    secondary_unit_price: u32,
    selected_provider_code: Option<String>,
    selected_shop_is_bulk: bool,
    start_step_value: Option<f64>,
    unit_price: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HoferCategory {
    #[serde(rename = "CategoryProviderImage")]
    category_provider_image: Option<String>,
    #[serde(rename = "alternativeCategoryProviderID")]
    alternative_category_provider_id: Option<String>,
    #[serde(rename = "brandProviderID")]
    brand_provider_id: u32,
    #[serde(rename = "categoryID")]
    category_id: u32,
    category_icon_file_name: Option<String>,
    category_image_file_name: Option<String>,
    category_name: String,
    child_list: Vec<String>,
    is_root: bool,
    level: u32,
    parent_category: Option<String>,
    #[serde(rename = "progID")]
    prog_id: String,
    url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HoferIcon {
    #[serde(rename = "blobURL")]
    blob_url: Option<String>,
    icon_tooltip: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HoferProductDetails {
    alcohol_str: Option<String>,
    allergenic: Option<String>,
    consumption: String,
    description: String,
    display_unit: String,
    ingrdients: Option<String>,
    manufacturer: Option<String>,
    origin_country: Option<String>,
    package_type: String,
    producer_address: String,
    promotion: Option<String>,
    secondary_unit: Option<String>,
    secondary_unit_price: u32,
    service_department: String,
    storing: String,
    supplier_address: String,
    supplier_phone: String,
    supplier_web: String,
    textual_nutrition: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HoferProductMediaItem {
    #[serde(rename = "MediaUrlL")]
    media_url_large: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HoferProductProvider {
    available: bool,
    deposit_fee: Option<f64>,
    display_unit: String,
    is_bulk: bool,
    is_offer: bool,
    is_one_product_for_free: bool,
    max_weight_step: f64,
    min_weight_step: f64,
    offer_valid_to: Option<String>,
    original_price_if_offer: Option<String>,
    original_unit_price_if_offer: Option<String>,
    package_base_unit: String,
    package_quantity: Number,
    price: f64,
    product: Option<String>,
    #[serde(rename = "productID")]
    product_id: u32,
    #[serde(rename = "productProviderProviderDepositProducList")]
    product_provider_deposit_list: Vec<String>,
    #[serde(rename = "providerID")]
    provider_id: u32,
    provider_product_image_url: String,
    provider_product_name: String,
    roksh_discount_price: Option<String>,
    secondary_unit: Option<String>,
    secondary_unit_price: u32,
    shop_available: Option<String>,
    unit: Option<String>,
    unit_price: String,
    unit_price_int: u32,
}
