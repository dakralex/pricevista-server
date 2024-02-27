use crate::parser;
use crate::providers::{Fetch, Merge};
use reqwest::Client;
use serde::{Deserialize, Deserializer};
use serde_json::Number;
use std::str::FromStr;

pub struct BillaShopApi;

impl BillaShopApi {
    const PAGE_SIZE: &'static str = "500";
    const STORE_ID: &'static str = "00-10";

    async fn request(client: &Client, page: usize) -> crate::error::Result<BillaSearchResponse> {
        let url = format!("{}/products", Self::API_BASE_URL);

        let request = client.get(&url).query(&[
            ("page", page.to_string().as_str()),
            ("pageSize", Self::PAGE_SIZE),
            ("storeId", Self::STORE_ID),
        ]);
        let response = request.send().await?;
        Ok(response.json::<BillaSearchResponse>().await?)
    }
}

impl Fetch for BillaShopApi {
    type ResponseImpl = BillaSearchResponse;
    const API_BASE_URL: &'static str = "https://shop.billa.at/api";

    async fn fetch(client: &Client) -> crate::error::Result<Self::ResponseImpl> {
        let mut page = 0;
        let mut total = Self::request(&client, page).await?;

        while let Some(count) = &total.count {
            // TODO Find out if this check is needed
            if count == &0 {
                break;
            }

            page += 1;
            total.merge(Self::request(&client, page).await?);
        }

        Ok(total)
    }
}

#[derive(Debug, Deserialize)]
pub struct BillaSearchResponse {
    /// The count of the items that are contained in results (if any).
    count: Option<usize>,

    /// The total amount of item the request produces (through paging).
    total: usize,

    /// The results from the request.
    results: Vec<BillaProduct>,

    /// The facets that are relevant for the request.
    facets: Vec<BillaFacet>,
}

impl Merge for BillaSearchResponse {
    fn merge(&mut self, rhs: Self) {
        self.count = rhs.count;
        assert_eq!(self.total, rhs.total);
        self.results.extend(rhs.results);
        // TODO Make facets a HashSet or reimplement Extend/Trait on it
        self.facets.extend(rhs.facets);
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillaProduct {
    /// The age required in months to be allowed to buy the product.
    ///
    /// This field is either `Some(216)` (for 18 years old) or `None`.
    age_required_in_months: Option<u32>,

    /// The product's net quantity.
    ///
    /// The unit of this field is specified by the `volume_label_short` or
    /// `volume_label_key`/`volume_label_long`.
    #[serde(with = "parser::double_as_string")]
    amount: f64,

    /// The list of badges that apply to the product.
    #[serde(default)]
    badges: Vec<BillaBadge>,

    /// The product's brand information.
    ///
    /// This field is sometimes missing or inconsistent (where 2% of Billa
    /// products do not have any information here), it is still reliable for
    /// most products.
    brand: Option<BillaBrand>,

    /// The name of the product's main category.
    category: String,

    /// The product's country of origin.
    ///
    /// This is either a single country name or a comma-separated list of
    /// countries where the product originated from.
    country_of_origin: Option<String>,

    /// The product's container-deposit type.
    ///
    /// This field is mostly intended for bottles and cans.
    deposit_type: Option<BillaDepositType>,

    /// The product's short description.
    ///
    /// This field is shown in the online store's product page at the top and
    /// may contain HTML for better formatting.
    description_short: Option<String>,

    /// The product's long description.
    ///
    /// This field is shown in the bottom accordion in the online store's page
    /// and may contain HTML for better formatting.
    description_long: Option<String>,

    /// The product's drained weight.
    ///
    /// This field is used for products that are in some container and contain
    /// an excess of some kind of preservation liquid.
    drained_weight: Option<f64>,

    /// The product's image url list.
    ///
    /// This field is only rarely empty, but can contain more than one image url.
    images: Vec<String>,

    /// The product's maximum quantity that may be bought in the online store.
    max_quantity: Option<Number>,

    /// Whether the product is for medical purposes.
    medical: bool,

    /// The product's minimum quantity that must be bought in the online store.
    min_quantity: Option<Number>,

    /// The product's display name.
    name: String,

    /// The product's package label string.
    package_label: Option<BillaPackageLabel>,

    /// The product's package label key.
    ///
    /// This field describes how the package unit is referenced and called, e.g.
    /// a bottle of beer, a package of coffee beans, a cup of yoghurt.
    package_label_key: Option<BillaPackageLabel>,

    /// The product's categories in top-to-bottom order.
    ///
    /// This field contains one or more flat category paths (outer vector),
    /// which are in the order of top-to-bottom (inner vector), e.g.
    /// "Kühlwaren", "Feinkost", "Wurst" (name values only)
    parent_categories: Vec<Vec<BillaParentCategory>>,

    /// The product's preparation time.
    ///
    /// This field describes the time it takes to cook, prepare and/or bake the
    /// product by store employees. This only applies to specific products,
    /// which are e.g. made for party deliveries.
    preparation_time: Option<Number>,

    /// The product's price information.
    price: BillaPriceInfo,

    /// The product's unique product identifier.
    product_id: String,

    /// The product's marketing text.
    ///
    /// This field value seems unused in the online store.
    product_marketing: Option<String>,

    /// Whether the product has been purchased (when logged in).
    purchased: bool,

    /// The product's quantity step size for the selector in the online store.
    ///
    /// This field is used as the amount to step by when pressing the decrement
    /// and increment buttons to adjust the quantity to buy.
    ///
    /// If the product's `weight_article` field is `true` and the
    /// `volume_label_short` field is `Kilogram`, then the quantity step size
    /// is in grams (g) and needs to be converted by dividing through `1000` to
    /// get the step size in kilograms (kg).
    quantity_step_size: Option<Number>,

    /// The product's unique stock keeping unit identifier.
    sku: String,

    /// The product's unique slug that is made out of the `name` and `sku`.
    ///
    /// This field can be used to generate a permanent link to the product page
    /// at `https://shop.billa.at/produkte/{slug}`, which can be further used
    /// for scraping additional data that is not provided through the API.
    slug: String,

    upsell_sku: Option<String>,

    /// The product's volume label key.
    ///
    /// This field specifies the `amount` field measurement unit and therefore
    /// in which measurement unit the product is counted in. If this field does
    /// not have the same value as the `volume_label_short` field, it is an
    /// arbitrary unit (e.g. 20 tea bags instead of 60 g of tea).
    volume_label_key: BillaVolumeLabel,

    /// The product's long volume label string.
    volume_label_long: BillaVolumeLabel,

    /// The product's short volume label string.
    ///
    /// This field can only be a metric measurement unit and specifies the unit
    /// of the `amount` field.
    volume_label_short: Option<BillaVolumeLabel>,

    /// The product's gross weight in kilograms (kg).
    weight: f64,

    /// Whether the product is sold by weight.
    ///
    /// This field is set to `true` for products like fruits, vegetables, meat,
    /// cheese, sausages, etc. If the `volume_label_short` field is `"kg"`, then
    /// this product is sold per kilogram.
    weight_article: bool,

    weight_per_piece: f64,
    weight_piece_article: bool,
}

/// Enumeration of the different badges used in the BILLA Online Shop.
///
/// These badges are used to indicate some quality of a product, that is either
/// some private or public sign of quality (e.g. FairTrade, Rainforest, DOP),
/// a hint for a specific diet (e.g. vegan, gluten-free), a hint for how the
/// product should be stored (e.g. deep-frozen) or something more ambigous
/// (e.g. new product, product from Austria).
#[derive(Debug, Deserialize)]
enum BillaBadge {
    /// Austrian "AMA Gütesiegel" organic food certification
    #[serde(rename = "gs-amabio")]
    AMAMark,

    /// French/Swiss "Appellation d’Origine Contrôlée" certification
    #[serde(rename = "gs-aoc")]
    AOCMark,

    /// French/Swiss "Appellation d’Origine Protégée" certification
    #[serde(rename = "gs-aop")]
    AOPMark,

    /// Austrian "ARGE Heumilch" certification
    #[serde(rename = "gs-argeheu")]
    ArgeMark,

    /// International Aquaculture Stewardship Council certification
    #[serde(rename = "gs-asc")]
    ASCMark,

    /// (Generic?) organic food certification
    #[serde(rename = "gs-bio")]
    BioMark,

    /// International Demeter organic food certification
    #[serde(rename = "gs-demeter")]
    DemeterMark,

    /// Italian "Denominazione di'Origine controllata" certification
    #[serde(rename = "gs-doc")]
    DOCMark,

    /// Italian "Denominazione di'Origine Protetta" certification
    #[serde(rename = "gs-dop")]
    DOPMark,

    /// European Union's organic food certification
    #[serde(rename = "gs-eubio")]
    EUBioMark,

    /// Austrian "Fair zum Tier" animal certification
    #[serde(rename = "gs-fairzumtier")]
    FairZumTier,

    /// International Forest Stewardship Council certification
    #[serde(rename = "gs-fsc")]
    FSCMark,

    /// International FairTrade certification
    #[serde(rename = "gs-ft")]
    FairTradeMark,

    /// Generic certification for products that are free from genetic engineering
    #[serde(rename = "gs-gentech")]
    GenTechMark,

    /// European "Protected Geographical Indication" certification
    ///
    /// - German: geschützte geografische Angabe (ggA)
    /// - French: indication géographique protégée (IGP)
    /// - Italian: indicazione geografica protetta (IGP)
    /// - Portuguese: indicação geográfica protegida (IGP)
    /// - Spanish: indicación geográfica protegida (IGP)
    #[serde(rename = "gs-gga")]
    PGIMark,

    /// European "Traditional Speciality Guaranteed" certification
    ///
    /// - German: garantiert traditionelle Spezialität (gtS)
    /// - French: spécialité traditionnelle garantie (STG)
    /// - Italian: specialità tradizionale garantita (STG)
    /// - Spanish: especialidad tradicional garantizada (ETG)
    ///
    /// Note: The Billa API has probably an spelling error ("gta" instead of "gts")
    #[serde(rename = "gs-gta")]
    TSGMark,

    /// European "Protected Designation of Origin" certification
    ///
    /// - German: geschützte Ursprungsbezeichnung (gU)
    /// - French: Appellation d’Origine Contrôlée (AOP)
    /// - Italian: Denominazione d’Origine Protetta (DOP)
    /// - Portuguese: Denominação de Origem Protegida (DOP)
    /// - Spanish: denominación de origen protegida (DOP)
    #[serde(rename = "gs-gu")]
    PDOMark,

    /// International Marine Stewardship Council certification
    #[serde(rename = "gs-msc")]
    MSCMark,

    /// Proprietary PRO PLAN pet food certification
    #[serde(rename = "gs-proplan")]
    ProPlanMark,

    /// International Rainforest Alliance certification
    #[serde(rename = "gs-rainfor")]
    RainforestMark,

    /// International UTZ Certified certification
    #[serde(rename = "gs-utz")]
    UTZCertifiedMark,

    /// Badge for vegan products
    ///
    /// This badge is an alternative to [BillaBadge::VeganProduct].
    #[serde(rename = "gs-vegan")]
    VeganMark,

    /// Label for products originating from Austria
    #[serde(rename = "homecountry")]
    AustrianProduct,

    /// Label for organic products
    ///
    /// This badge is an alternative to [BillaBadge::BioMark].
    #[serde(rename = "pp-bio")]
    BioProduct,

    /// Label for cooled products
    #[serde(rename = "kuehlung")]
    CooledProduct,

    /// In-store label for deep frozen products
    #[serde(rename = "tiefkuehlung")]
    DeepFrozenProduct,

    /// Label for gluten-free products
    #[serde(rename = "pp-glutenfr")]
    GlutenFreeProduct,

    /// Label for lactose-free products
    #[serde(rename = "pp-lakto")]
    LactoseFreeProduct,

    /// In-store label for new products
    #[serde(rename = "eg-neu")]
    NewProduct,

    /// Label for vegan products
    ///
    /// This badge is an alternative to [BillaBadge::VeganMark].
    #[serde(rename = "pp-vegan")]
    VeganProduct,

    /// Label for vegetarian products
    #[serde(rename = "pp-veget")]
    VegetarianProduct,
}

#[derive(Debug, Deserialize)]
struct BillaBrand {
    /// The brand's name.
    name: String,

    /// The brand's unique slug.
    slug: String,
}

/// Enumeration of the different ways to deposit a container.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum BillaDepositType {
    /// The product is oneway disposable and therefore should be discarded in
    /// through a general waste management system.
    Oneway,

    /// The product is reusable and should be brought back to the store to
    /// recycle it through the supermarket's logistics.
    Reusable,

    #[serde(other)]
    Unknown,
}

/// Enumeration of the package labels, which describes how a single product
/// item is counted.
#[derive(Debug, Deserialize)]
enum BillaPackageLabel {
    #[serde(rename = "bd")]
    #[serde(alias = "Bund")]
    Bunch,

    #[serde(rename = "be")]
    #[serde(alias = "Becher")]
    Cup,

    #[serde(rename = "bl")]
    #[serde(alias = "Blister")]
    Blister,

    #[serde(rename = "br")]
    #[serde(alias = "Brief")]
    Letter,

    #[serde(rename = "bt")]
    #[serde(alias = "Beutel")]
    Bag,

    #[serde(rename = "bx")]
    #[serde(alias = "gk")]
    #[serde(alias = "sc")]
    #[serde(alias = "Box")]
    #[serde(alias = "Geschenkkarton")]
    #[serde(alias = "Schachtel")]
    Box,

    #[serde(rename = "ct")]
    #[serde(alias = "Blechdose (ITA)")]
    TinCan,

    #[serde(rename = "ds")]
    #[serde(alias = "Dose")]
    Can,

    #[serde(rename = "fl")]
    #[serde(alias = "Flasche")]
    Bottle,

    #[serde(rename = "gl")]
    #[serde(alias = "Glas")]
    Glas,

    #[serde(rename = "ka")]
    #[serde(alias = "Kanister")]
    Canister,

    #[serde(rename = "kb")]
    #[serde(alias = "Korb")]
    Basket,

    #[serde(rename = "kg")]
    #[serde(alias = "Kilo")]
    Kilogram,

    #[serde(rename = "ki")]
    #[serde(alias = "Kiste")]
    Crate,

    #[serde(rename = "kt")]
    #[serde(alias = "Karton")]
    Carton,

    #[serde(rename = "ku")]
    #[serde(alias = "Kübel")]
    Bucket,

    #[serde(rename = "ne")]
    #[serde(alias = "nt")]
    #[serde(alias = "Netz")]
    Net,

    #[serde(rename = "pa")]
    #[serde(alias = "pk")]
    #[serde(alias = "pt")]
    #[serde(alias = "Packung")]
    #[serde(alias = "Paket")]
    Package,

    #[serde(rename = "pl")]
    #[serde(alias = "Scheibe(SLK)")]
    Slice,

    #[serde(rename = "rg")]
    #[serde(alias = "Riegel")]
    Bar,

    #[serde(rename = "rl")]
    #[serde(alias = "Rolle")]
    Roll,

    #[serde(rename = "sa")]
    #[serde(alias = "Sack")]
    Sack,

    #[serde(rename = "se")]
    #[serde(alias = "ta")]
    #[serde(alias = "Schale")]
    #[serde(alias = "Tasse")]
    Tray,

    #[serde(rename = "sp")]
    #[serde(alias = "Spray")]
    Spray,

    #[serde(rename = "st")]
    #[serde(alias = "Stück")]
    Piece,

    #[serde(rename = "tb")]
    #[serde(alias = "Tube")]
    Tube,

    #[serde(rename = "tf")]
    #[serde(alias = "tu")]
    #[serde(alias = "Tafel")]
    ChocolateBar,

    #[serde(rename = "tg")]
    #[serde(alias = "Tiegel")]
    Jar,

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BillaParentCategory {
    /// The category's five-digit key.
    key: String,

    /// The category's display name.
    name: String,

    /// The category's slug composed of the kebab-cased name and key.
    slug: String,
    order_hint: String,
}

/// Type of a Billa price value.
///
/// The value is given in the minor unit of the currency (that is, Euro cents).
#[derive(Debug, Deserialize)]
struct BillaPrice(u32);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BillaPriceInfo {
    /// The quantity that is used for calculating the bulk price.
    ///
    /// If this field value is `None`, there is no bulk price displayed in the
    /// online store.
    base_price_factor: Option<String>,

    /// The display name of the base unit for the bulk price.
    base_unit_long: String,

    /// The label of the base unit for the bulk price (e.g. 5.99 € / kg).
    base_unit_short: String,

    /// The original price value, if there is a discount promotion present.
    crossed: Option<BillaPrice>,

    /// The discount percentage as a negative integer value, if there is a
    /// discount promotion present.
    ///
    /// This field's value seems to be calculated with
    /// `floor((crossed  - regular.value) / crossed)`.
    discount_percentage: Option<i32>,

    /// The lowest previously recorded price?
    ///
    /// This field's value is not well studied and seems unreliable, but as far
    /// as we know it records the lowest previously recorded price. For products
    /// that are currently not discounted the current `regular.value` is never
    /// lower than this, but for products that are currently discounted the
    /// `crossed` and `regular` value is often much lower than this value.
    lowest_price: Option<BillaPrice>,

    /// More information for the price.
    regular: BillaPriceItem,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BillaPriceItem {
    /// The bulk price's current value.
    ///
    /// This is either the regular bulk price if there is no promotion present
    /// or the price that is applied if the promotion's requirements are
    /// fulfilled.
    per_standardized_quantity: BillaPrice,

    /// The promotion's minimum quantity of product items that need to be bought
    /// so that the promotion's discount can be applied.
    promotion_quantity: Option<BillaPrice>,

    /// The promotion's text that is displayed in the online store.
    promotion_text: Option<String>,

    /// The promotion's type.
    promotion_type: Option<BillaPromotionType>,

    /// The price's value if the promotion cannot be applied.
    promotion_value: Option<BillaPrice>,

    /// The bulk price's value if the promotion cannot be applied.
    promotion_value_per_standardized_quantity: Option<BillaPrice>,

    /// The price's promotion tags that
    tags: Vec<BillaPromotionTag>,

    /// The price's current value.
    ///
    /// This is either the regular price if there is no promotion present or
    /// the price that is applied if the promotion's requirements are fulfilled.
    value: BillaPrice,
}

/// Enumeration of Billa promotion types for a price value.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum BillaPromotionType {
    /// A promotion with a minimum required quantity that needs to be bought to
    /// apply the promotion's discount.
    ///
    /// The total price depends whether the `promotion_quantity` has been
    /// reached or not. If it was, then the total price is calculated with
    /// `value * quantity`, else it is `promotion_value * quantity`.
    From,

    /// A promotion with a minimum required quantity that needs to be bought to
    /// apply the promotion's discount and further is only fully applied if the
    /// quantity is a multiple of the `promotion_quantity`.
    ///
    /// The total price is calculated with
    /// `value * floor(quantity / promotion_quantity) + promotion_value * (quantity % promotion_quantity`.
    PerSetOf,

    #[serde(other)]
    Unknown,
}

#[derive(Debug)]
enum BillaPromotionTag {
    Kurant,

    Multi,

    Discount,

    /// A buy-one-get-one-free promotion type.
    ///
    /// This promotion has a variable `x + y` scheme, where `x` is the amount
    /// that needs to be bought to get `y` amount for _free_.
    Bogo(u32, u32),

    Unknown,
}

impl BillaPromotionTag {
    fn extract_bogo(term: &str) -> BillaPromotionTag {
        let nums: Vec<&str> = term.split("plus").collect();
        assert_eq!(nums.len(), 2);

        match (u32::from_str(nums[0]), u32::from_str(nums[1])) {
            (Ok(x), Ok(y)) => Self::Bogo(x, y),
            _ => Self::Unknown,
        }
    }
}

impl<'de> Deserialize<'de> for BillaPromotionTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        if let Some(suffix) = value.strip_prefix("pt-") {
            match suffix {
                "kurant" => Ok(Self::Kurant),
                "multi" => Ok(Self::Multi),
                "abverkauf" | "aktion" | "extrem" | "satterrabatt" => Ok(Self::Discount),
                s if s.contains("plus") => Ok(Self::extract_bogo(s)),
                _ => Ok(Self::Unknown),
            }
        } else {
            Err(serde::de::Error::custom("Invalid Billa promotion tag"))
        }
    }
}

#[derive(Debug, Deserialize)]
enum BillaVolumeLabel {
    #[serde(rename = "bd")]
    #[serde(alias = "Bund")]
    Bunch,

    #[serde(rename = "bl")]
    #[serde(alias = "Blatt")]
    Sheet,

    #[serde(rename = "bt")]
    #[serde(alias = "Beutel")]
    Bag,

    #[serde(rename = "cm")]
    #[serde(alias = "Zentimeter")]
    Centimetre,

    #[serde(rename = "gr")]
    #[serde(alias = "g")]
    #[serde(alias = "Gramm")]
    Gram,

    #[serde(rename = "kg")]
    #[serde(alias = "Kilogramm")]
    Kilogram,

    #[serde(rename = "lt")]
    #[serde(alias = "liter")]
    #[serde(alias = "Liter")]
    Litre,

    #[serde(rename = "ml")]
    #[serde(alias = "Milliliter")]
    Millilitre,

    #[serde(rename = "mt")]
    #[serde(alias = "m")]
    #[serde(alias = "Meter")]
    Metre,

    #[serde(rename = "pa")]
    #[serde(alias = "Paar")]
    Pair,

    #[serde(rename = "pk")]
    #[serde(alias = "Packung")]
    Package,

    #[serde(rename = "pt")]
    #[serde(alias = "Portion")]
    Portion,

    #[serde(rename = "rl")]
    #[serde(alias = "Rollen")]
    Roll,

    #[serde(rename = "st")]
    #[serde(alias = "stk")]
    #[serde(alias = "Stück")]
    Piece,

    #[serde(rename = "tb")]
    #[serde(alias = "Teebeutel")]
    Teabag,

    #[serde(rename = "wg")]
    #[serde(alias = "Waschgang")]
    Washload,

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BillaResultFacet {
    key: String,
    label: String,
    is_multi_select: bool,
    terms: Option<Vec<BillaFacetTerm>>,
    comparison_terms: Option<Vec<BillaFacetTerm>>,
    range: Option<BillaFacetRange>,
}

#[derive(Debug, Deserialize)]
struct BillaFacetTerm {
    key: String,
    label: String,
    term: String,
    count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BillaFacetRange {
    key: String,
    label: String,
    count: usize,
    label_symbol: String,
    min: f64,
    max: f64,
}
