use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct KitchenMetaData {
    pub known_supported_emoji: Vec<String>,
    pub data: HashMap<String, EmojiItem>,
}

#[derive(Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmojiItem {
    pub alt: String,
    pub emoji: String,
    pub emoji_codepoint: String,
    pub g_board_order: u64,
    pub keywords: Vec<String>,
    pub category: String,
    pub subcategory: String,
    pub combinations: Vec<Combination>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Combination {
    pub g_static_url: String,
    pub alt: String,
    pub left_emoji: String,
    pub left_emoji_codepoint: String,
    pub right_emoji: String,
    pub right_emoji_codepoint: String,
    pub date: String,
}
