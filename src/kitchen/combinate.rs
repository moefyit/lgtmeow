use crate::kitchen::metadata::{Combination, KitchenMetaData};

pub fn combinate_emojis(
    left_emoji_codepoint: &str,
    right_emoji_codepoint: &str,
    metadata: &KitchenMetaData,
) -> Option<Combination> {
    if !metadata
        .known_supported_emoji
        .contains(&left_emoji_codepoint.to_string())
        || !metadata
            .known_supported_emoji
            .contains(&right_emoji_codepoint.to_string())
    {
        return None;
    }
    if let Some(emoji_item) = metadata.data.get(left_emoji_codepoint) {
        if emoji_item.combinations.contains_key(right_emoji_codepoint)
            && !emoji_item.combinations[right_emoji_codepoint].is_empty()
        {
            return Some(emoji_item.combinations[right_emoji_codepoint][0].clone());
        }
    }
    None
}
