use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// The [Country] entity describes a country that is represented in the
/// ISO 3166-1 standard definition.
#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    /// ISO 3166-1 numeric (three-digit) country code
    pub id: usize,

    /// ISO 3166-1 alpha-2 two-letter country code
    pub alpha2: String,

    /// ISO 3166-1 alpha-3 three-letter country code
    pub alpha3: String,

    /// English name of the country
    pub name: String,
}

/// The [Currency] entity describes a currency that is represented in the
/// ISO 4217 standard definition.
#[derive(Debug, Deserialize, Serialize)]
pub struct Currency {
    /// ISO 4217 (three-digit) currency code
    pub id: usize,

    /// ISO 4217 three-letter currency code
    pub alpha3: String,

    /// Number of digits for the subunit
    pub scale: usize,

    /// Symbol of the currency
    pub symbol: String,

    /// Symbol of the currency's subunit
    pub minor: Option<String>,

    /// English name of the currency
    pub name: String,
}

/// The [Language] entity describes a language that is represented in the
/// ISO 639-1 standard definition.
#[derive(Debug, Deserialize, Serialize)]
pub struct Language {
    /// ISO 639-1 three-digit language code
    pub id: usize,

    /// ISO 639-1 two-letter language code
    pub alpha2: String,

    /// ISO 639-2 three-letter language code
    pub alpha3: String,

    /// English name of the language
    pub name: String,
}

/// The [MeasurementUnit] entity describes a measurement unit that is used by
/// stores to quantify their package and volume sizes.
#[derive(Debug, Deserialize, Serialize)]
pub struct MeasurementUnit {
    pub id: usize,

    /// Symbol of the measurement unit
    pub symbol: String,

    /// English name of the measurement unit
    pub name: String,
}

/// The [Place] entity describes a location in the physical world.
#[derive(Debug, Deserialize, Serialize)]
pub struct Place {
    pub id: usize,

    /// Country of the place
    pub country: Country,

    /// Administrative area of the place (e.g. region, county, state)
    #[serde(rename = "adminArea")]
    pub admin_area: Option<String>,

    /// Locality of the place (e.g. city, municipality, town)
    pub locality: Option<String>,

    /// Postal code of the place
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,

    // Street address line of the place
    #[serde(rename = "streetAddress")]
    pub street_address: Option<String>,
}

/// The [Company] entity is the generalization of the different companies that
//  are somewhere in the supply chain. In this micro-world, they are retailers
//  and brands.
#[derive(Debug, Deserialize, Serialize)]
pub struct Company {
    pub id: usize,

    /// Full name of the company
    #[serde(rename = "longName")]
    pub long_name: Option<String>,

    /// Short name of the company
    #[serde(rename = "shortName")]
    pub short_name: String,

    /// Physical location of the company's headquarters
    pub place: Option<Place>,
}

/// The [CompanyParticipation] relation describes the relationship between
//  stakeholders and companies they take interest in.
#[derive(Debug, Deserialize, Serialize)]
pub struct CompanyParticipation {
    /// Stakeholder that takes interest in a company
    pub stakeholder: Company,

    /// The company that the stakeholder takes interest in
    pub company: Company,
}

/// The [Retailer] entity describes a specialization of a [Company], which
//  functions as a retailer of one or more stores that sell articles.
#[derive(Debug, Deserialize, Serialize)]
pub struct Retailer {
    /// Company of the retailer
    pub company: Company,

    /// URL address to the main online presence of the retailer
    #[serde(rename = "websiteUrl")]
    pub website_url: Option<String>,
}

/// The [Brand] entity describes a specialization of a [Company], which
/// functions as a brand that brand articles that are sold in stores.
#[derive(Debug, Deserialize, Serialize)]
pub struct Brand {
    /// Company of the brand
    pub company: Company,

    /// URL address to the logo of the brand
    #[serde(rename = "logoUrl")]
    pub logo_url: Option<String>,
}

/// The [Category] entity describes a category that articles can belong to.
#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: usize,

    /// Name of the category
    pub name: String,

    /// Description of the category
    pub description: Option<String>,
}

/// The [Article] entity describes an article that is available in one or more
//  stores of the retailers with some details.
#[derive(Debug, Deserialize, Serialize)]
pub struct Article {
    pub id: usize,

    /// Brand of the article
    pub brand: Option<Brand>,

    /// Name of the article without brand, quantity or unit
    pub name: String,

    /// Description of the article
    pub description: Option<String>,

    /// Country where the article originated from
    #[serde(rename = "originCountry")]
    pub origin_country: Option<Country>,

    /// Measurement unit of the article
    pub unit: MeasurementUnit,

    /// Quantity of the article in the measurement unit
    pub quantity: Decimal,

    /// Whether the article can be bought in bulk by weighing it
    pub weightable: bool,
}

/// The [ArticleCategory] relation describes the categories an article
//  belongs to.
#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleCategory {
    /// Article that is categorized
    pub article: Article,

    /// Category the article belongs to
    pub category: Category,
}

/// The [ArticleImage] relation describes which images of the articles are known.
#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleImage {
    /// Article the image references to
    pub article: Article,

    pub id: usize,

    /// URL address to some image of the article
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}

/// The [Store] entity describes a place where a retailer distributes articles
//  by advertising, stocking, and selling them over that platform. This could
//  be a physical place (e.g. a shop) or a virtual place (e.g. online store).
//
//  In the case that this store is an online store, the location should at least
//  define the country the online store is intended for.
#[derive(Debug, Deserialize, Serialize)]
pub struct Store {
    pub id: usize,

    /// Retailer that owns the store
    pub retailer: Retailer,

    /// Location of the store
    pub location: Place,

    /// Currency of the store
    pub currency: Currency,

    /// Language of the store
    pub language: Language,
}

/// The [StoreArticleMap] relation describes the mapping between the store's
//  internal article identifiers to the articles.
#[derive(Debug, Deserialize, Serialize)]
pub struct StoreArticleMap {
    /// Store that holds the article
    pub store: Store,

    /// Internal article identifier used by the store
    #[serde(rename = "storeArticleId")]
    pub store_article_id: String,

    /// Article that the store's internal article identifier references
    pub article: Article,

    /// Timestamp when the article was first discovered (e.g. API access time,
    /// JSON import date, timestamp property in the API)
    pub since: DateTime<Utc>,
}

/// The [StoreCategoryMap] relation describes the mapping between the store's
//  internal category identifiers to the categories.
#[derive(Debug, Deserialize, Serialize)]
pub struct StoreCategoryMap {
    /// Store that has the category
    pub store: Store,

    /// Category identifier used by the store internally
    #[serde(rename = "storeCategoryId")]
    pub store_category_id: String,

    /// Category that the store's internal category identifier references
    pub category: Category,
}

/// The [CurrentPrice] relation describes the current price of the articles at
//  different stores.
#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentPrice {
    /// Store of the current price
    pub store: Store,

    /// Article of the current price
    pub article: Article,

    /// Value of the current price in the store's currency's minor unit
    pub value: u64,

    /// Timestamp when the current price was changed at
    #[serde(rename = "changedAt")]
    pub changed_at: DateTime<Utc>,

    /// Timestamp when the current price was updated at
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

/// The [RecordedPrice] relation describes the historical prices of articles
//  at specific stores.
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordedPrice {
    /// Store of the recorded price
    pub store: Store,

    /// Article of the recorded price
    pub article: Article,

    /// Timestamp when the price was recorded
    #[serde(rename = "changedAt")]
    pub changed_at: DateTime<Utc>,

    /// Value of the recorded price in the store's currency's minor unit
    pub value: u64,
}
