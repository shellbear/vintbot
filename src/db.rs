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
    pub search_text: Option<String>,
    #[serde(rename = "size_id[]")]
    pub sizes: Option<Vec<i32>>,
    #[serde(rename = "catalog[]")]
    pub categories: Option<Vec<i32>>,
    #[serde(rename = "status_ids[]")]
    pub conditions: Option<Vec<i32>>,
    // https://www.vinted.fr/api/v2/brands?keyword=
    #[serde(rename = "brand_id[]")]
    pub brands: Option<Vec<i32>>,
    pub price_from: Option<i32>,
    pub price_to: Option<i32>,
}

impl ItemsFilters {
    pub fn to_query(&self) -> Vec<(&str, serde_json::Value)> {
        let mut query: Vec<(&str, serde_json::Value)> = Vec::new();

        if let Some(search_text) = &self.search_text {
            query.push(("search_text", serde_json::Value::from(search_text.clone())));
        }
        if let Some(sizes) = &self.sizes {
            for size in sizes {
                query.push(("size_id[]", serde_json::Value::from(*size)));
            }
        }
        if let Some(categories) = &self.categories {
            for category in categories {
                query.push(("catalog[]", serde_json::Value::from(*category)));
            }
        }
        if let Some(conditions) = &self.conditions {
            for condition in conditions {
                query.push(("status_ids[]", serde_json::Value::from(*condition)));
            }
        }
        if let Some(brands) = &self.brands {
            for brand in brands {
                query.push(("brand_id[]", serde_json::Value::from(*brand)));
            }
        }
        if let Some(price_from) = self.price_from {
            query.push(("price_from", serde_json::Value::from(price_from)));
        }
        if let Some(price_to) = self.price_to {
            query.push(("price_t", serde_json::Value::from(price_to)));
        }

        query
    }
}

impl Default for ItemsFilters {
    fn default() -> Self {
        Self {
            search_text: None,
            sizes: None,
            categories: None,
            conditions: None,
            brands: None,
            price_from: None,
            price_to: None,
        }
    }
}

pub enum Sizes {
    XS = 206,
    S = 207,
    M = 208,
    L = 209,
    XL = 210,
    XXL = 211,
    XXXL = 212,
}

pub enum Categories {
    Men = 5,
    Clothes = 2050,
}

pub enum Condition {
    NewWithoutTags = 1,
    ReallyGood = 2,
    Good = 3,
    Satisfactory = 4,
    NewWithTags = 6,
}
