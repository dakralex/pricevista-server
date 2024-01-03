use chrono::{DateTime, Utc};

pub struct Company {
    pub id: u32,
    pub name: String,
    pub country: String,
    pub admin_area: Option<String>,
    pub locality: Option<String>,
    pub postal_code: Option<String>,
    pub street_address: Option<String>,
}

pub struct CompanyOwnership {
    pub owner: Company,
    pub ownee: Company,
}

pub struct Retailer {
    pub market_share: Option<f64>,
    pub annual_revenue: Option<f64>,
    pub profit_margin: Option<f64>,
    pub working_currency: String,
}

pub struct Brand {
    pub product_line: Option<String>,
}

pub struct Category {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}

pub struct Article {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub origin_country: Option<String>,
    pub image_url: Option<String>,
    pub brand: Option<Brand>,
    pub category: Option<Category>,
}

pub struct ArticleVariant {
    pub id: u32,
    pub article: Article,
    pub unit: String,
    pub quantity: f64,
    pub weightable: bool,
}

pub struct Store {
    pub id: u32,
    pub retailer: Retailer,
    pub url_address: Option<String>,
    pub country: String,
    pub admin_area: Option<String>,
    pub locality: Option<String>,
    pub postal_code: Option<String>,
    pub street_address: Option<String>,
}

pub struct StoreArticleMap {
    pub store: Store,
    pub store_article_id: String,
    pub article_variant: ArticleVariant,
    pub since: DateTime<Utc>,
}

pub struct StoreCategoryMap {
    pub store: Store,
    pub store_category_id: String,
    pub category: Category,
}
