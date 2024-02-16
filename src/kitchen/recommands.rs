use super::metadata::EmojiItem;

/// Get a list of recommended combinations of 🐾
pub fn get_recommand_emoji_combinate_with_paw_prints() -> Vec<String> {
    vec![
        "2601-fe0f",  // ☁️
        "1f495",      // ❤️
        "2b50",       // ⭐
        "1f30a",      // 🌊
        "1f31f",      // 🌟
        "1f4ae",      // 🌸
        "1f381",      // 🎁
        "1f386",      // 🎆
        "1f38a",      // 🎊
        "1f3b0",      // 🎰
        "1f44d",      // 👍
        "1f495",      // 💕
        "1f493",      // 💓
        "1f498",      // 💘
        "1f4a1",      // 💡
        "1f4a5",      // 💥
        "1f4df",      // 📟
        "1f4f0",      // 📰
        "1f525",      // 🔥
        "1f52e",      // 🔮
        "1f5ef-fe0f", // 🗯️
        "1f6f8",      // 🛸
        "1f947",      // 🥇
        "1f9ca",      // 🧊
        "1faa4",      // 🪤
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

pub fn get_paw_prints_combinations() -> EmojiItem {
    // Run just pre-build to fetch emoji kitchen metadata from GitHub:
    // https://raw.githubusercontent.com/xsalazar/emoji-kitchen-backend/main/app/metadata.json
    // And prepare via jq to get the paw prints combinations
    let raw_json_data = include_str!("paw_prints_kitchen_data.json");
    let kitchen_data: EmojiItem = serde_json::from_str(raw_json_data).unwrap();
    kitchen_data
}
