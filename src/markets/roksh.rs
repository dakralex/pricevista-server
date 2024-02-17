use reqwest::Client;
use serde::{Deserialize, Serialize};

const API_URL_SESSION_CONFIGURE: &str = "https://shopservice.roksh.at/session/configure";
const API_URL_FULL_CATEGORY_LIST: &str =
    "https://shopservice.roksh.at/category/GetFullCategoryList";
const API_URL_CATEGORY_PRODUCT_LIST: &str =
    "https://shopservice.roksh.at/productlist/CategoryProductList";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct RokshSessionConfig<'a> {
    user_selected_shops: Vec<&'a str>,
    redirect_to_dashboard_needed: bool,
    set_user_selected_shops_on_first_site_load: bool,
    shops_selected_for_root: &'a str,
    own_webshop_provider_code: &'a str,
}

impl<'a> RokshSessionConfig<'a> {
    fn new(shop_name: &'a str) -> Self {
        Self {
            user_selected_shops: vec![],
            redirect_to_dashboard_needed: false,
            set_user_selected_shops_on_first_site_load: true,
            shops_selected_for_root: shop_name,
            own_webshop_provider_code: "",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RokshCategoryListResponse(Vec<RokshCategoryItem>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct RokshCategoryItem {
    #[serde(rename = "CategoryID")]
    category_id: u32,
    category_name: String,
    #[serde(rename = "ProgID")]
    prog_id: String,
    #[serde(rename = "ParentCategoryID")]
    parent_category_id: Option<u32>,
    status_flag: bool,
    child_list: Vec<Box<RokshCategoryItem>>,
    url: String,
    category_icon_file_name: String,
    category_image_file_name: String,
}

/// Returns a authorization bearer for a Roksh/WeShop API session.
///
/// This function makes a request to the Roksh/WeShop API to generate a session
/// bearer that is required to configure what data should be queried in further
/// API calls (e.g. which shops are selected).
pub async fn get_session_bearer(
    client: &Client,
    shop_name: &str,
) -> reqwest::Result<Option<String>> {
    let config = RokshSessionConfig::new(shop_name);
    let req = client
        .post(API_URL_SESSION_CONFIGURE)
        .json(&config)
        .send()
        .await?;
    let bearer = req.headers().get("jwt-auth");
    Ok(bearer.map(|b| b.to_str().unwrap_or_default().to_string()))
}

/// Returns the list of categories that were responded from a full category list
/// request from the Roksh/WeShop API.
pub async fn get_full_category_list(
    client: &Client,
    bearer: &str,
) -> reqwest::Result<RokshCategoryListResponse> {
    let request = client
        .post(API_URL_FULL_CATEGORY_LIST)
        .bearer_auth(bearer)
        .send()
        .await?;
    Ok(request.json::<RokshCategoryListResponse>().await?)
}

/// Returns the list of products that were responded from subsequent requests
/// from the Roksh/WeShop API with all the categories provided.
pub async fn get_category_product_list(client: &Client, bearer: &str) -> reqwest::Result<String> {
    // TODO Implement Flatten trait for CategoryListItem and loop through the
    //      Vec to retrieve all product items
    let req = client
        .get(API_URL_CATEGORY_PRODUCT_LIST)
        .bearer_auth(bearer)
        .query(&[
            ("progId", ""),
            ("firstLoadProductListResultNum", 4),
            ("listResultProductNum", 24),
        ])
        .send()
        .await?;

    Ok(req.text().await?)
}
