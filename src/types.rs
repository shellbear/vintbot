use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub current_page: u32,
    pub per_page: u32,
    pub total_entries: u32,
    pub total_pages: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub badge: Value,
    #[serde(rename = "brand_title")]
    pub brand_title: String,
    #[serde(rename = "content_source")]
    pub content_source: String,
    pub conversion: Value,
    pub currency: String,
    pub discount: Value,
    #[serde(rename = "favourite_count")]
    pub favourite_count: i64,
    #[serde(rename = "favourite_group_id")]
    pub favourite_group_id: Value,
    pub id: i64,
    #[serde(rename = "is_favourite")]
    pub is_favourite: bool,
    #[serde(rename = "is_for_swap")]
    pub is_for_swap: bool,
    #[serde(rename = "is_visible")]
    pub is_visible: i64,
    pub photo: Option<Photo>,
    pub price: String,
    #[serde(rename = "service_fee")]
    pub service_fee: String,
    #[serde(rename = "shipping_fee")]
    pub shipping_fee: Value,
    #[serde(rename = "size_title")]
    pub size_title: String,
    pub title: String,
    #[serde(rename = "total_item_price")]
    pub total_item_price: String,
    pub url: String,
    pub user: User,
    #[serde(rename = "view_count")]
    pub view_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    #[serde(rename = "dominant_color")]
    pub dominant_color: String,
    #[serde(rename = "dominant_color_opaque")]
    pub dominant_color_opaque: String,
    pub extra: Extra,
    #[serde(rename = "full_size_url")]
    pub full_size_url: String,
    pub height: i64,
    #[serde(rename = "high_resolution")]
    pub high_resolution: HighResolution,
    pub id: i64,
    #[serde(rename = "image_no")]
    pub image_no: i64,
    #[serde(rename = "is_hidden")]
    pub is_hidden: bool,
    #[serde(rename = "is_main")]
    pub is_main: bool,
    #[serde(rename = "is_suspicious")]
    pub is_suspicious: bool,
    pub thumbnails: Vec<Thumbnail>,
    pub url: String,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extra {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighResolution {
    pub id: String,
    pub orientation: Value,
    pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub height: i64,
    #[serde(rename = "original_size")]
    pub original_size: Option<bool>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub business: bool,
    pub id: i64,
    pub login: String,
    pub photo: Option<UserPhoto>,
    #[serde(rename = "profile_url")]
    pub profile_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPhoto {
    #[serde(rename = "dominant_color")]
    pub dominant_color: String,
    #[serde(rename = "dominant_color_opaque")]
    pub dominant_color_opaque: String,
    pub extra: Extra2,
    #[serde(rename = "full_size_url")]
    pub full_size_url: String,
    pub height: i64,
    #[serde(rename = "high_resolution")]
    pub high_resolution: HighResolution2,
    pub id: i64,
    #[serde(rename = "is_hidden")]
    pub is_hidden: bool,
    #[serde(rename = "is_suspicious")]
    pub is_suspicious: bool,
    pub orientation: Value,
    pub reaction: Value,
    #[serde(rename = "temp_uuid")]
    pub temp_uuid: Value,
    pub thumbnails: Vec<Thumbnail2>,
    pub url: String,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extra2 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighResolution2 {
    pub id: String,
    pub orientation: Value,
    pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail2 {
    pub height: i64,
    #[serde(rename = "original_size")]
    pub original_size: Value,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub width: i64,
}
