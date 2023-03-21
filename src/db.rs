use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug)]
struct Query {
    id: i32,
    last_index: i64,
    created_at: time::Time,
    updated_at: time::Time,
}

#[derive(Deserialize, Serialize)]
pub struct ItemsFilters {
    #[serde(rename = "size_id[]")]
    sizes: Option<Vec<i32>>,
    #[serde(rename = "catalog[]")]
    categories: Option<Vec<i32>>,
    #[serde(rename = "status_ids[]")]
    conditions: Option<Vec<i32>>,
    // https://www.vinted.fr/api/v2/brands?keyword=
    #[serde(rename = "brand_id[]")]
    brands: Option<Vec<i32>>,
    price_from: Option<i32>,
    price_to: Option<i32>,
}

impl Default for ItemsFilters {
    fn default() -> Self {
        Self {
            sizes: None,
            categories: None,
            conditions: None,
            brands: None,
            price_from: None,
            price_to: None,
        }
    }
}

enum Sizes {
    XS = 206,
    S = 207,
    M = 208,
    L = 209,
    XL = 210,
    XXL = 211,
    XXXL = 212,
}

enum Categories {
    Men = 5,
    Clothes = 2050,
}

enum Condition {
    NewWithoutTags = 1,
    ReallyGood = 2,
    Good = 3,
    Satisfactory = 4,
    NewWithTags = 6,
}
