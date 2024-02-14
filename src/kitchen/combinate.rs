use super::metadata::{Combination, KitchenMetaData};

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
        for combination in emoji_item.combinations.iter() {
            if combination.left_emoji_codepoint == right_emoji_codepoint
                || combination.right_emoji_codepoint == right_emoji_codepoint
            {
                return Some(combination.clone());
            }
        }
    }
    None
}
