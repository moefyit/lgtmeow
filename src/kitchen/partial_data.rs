use super::metadata::{EmojiItem, KitchenMetaData};

pub fn get_paw_prints_combinations() -> EmojiItem {
    // Run build.rs to fetch emoji kitchen metadata from GitHub:
    // https://raw.githubusercontent.com/xsalazar/emoji-kitchen-backend/main/app/metadata.json
    // And prepare via jq to get the paw prints combinations
    let raw_json_data = include_str!("paw_prints_kitchen_data.json");
    let kitchen_data: EmojiItem = serde_json::from_str(raw_json_data).unwrap();
    kitchen_data
}

pub fn reconstruct_metadata_from_partial_data(
    metadata: &mut KitchenMetaData,
    emoji_item: &EmojiItem,
) {
    // Collect all known supported emoji
    let mut known_supported_emoji_in_emoji_item = vec![];
    for combination in emoji_item.combinations.iter() {
        known_supported_emoji_in_emoji_item.push(combination.left_emoji_codepoint.clone());
        known_supported_emoji_in_emoji_item.push(combination.right_emoji_codepoint.clone());
    }
    // Push the known supported emoji into the metadata.known_supported_emoji
    for known_supported_emoji in known_supported_emoji_in_emoji_item.iter() {
        if !metadata
            .known_supported_emoji
            .contains(known_supported_emoji)
        {
            metadata
                .known_supported_emoji
                .push(known_supported_emoji.clone());
        }
    }

    // Push the emoji_item into the metadata.data
    metadata
        .data
        .entry(emoji_item.emoji_codepoint.clone())
        .or_insert(emoji_item.clone());

    // Reconstruct other emoji_item from the combination
    for combination in &emoji_item.combinations {
        let emoji_codepoint = emoji_item.emoji_codepoint.clone();
        let (other_emoji_codepoint, other_emoji) =
            if combination.left_emoji_codepoint == emoji_codepoint {
                (
                    combination.right_emoji_codepoint.clone(),
                    combination.right_emoji.clone(),
                )
            } else {
                (
                    combination.left_emoji_codepoint.clone(),
                    combination.left_emoji.clone(),
                )
            };

        metadata
            .data
            .entry(other_emoji_codepoint.clone())
            .or_insert(EmojiItem {
                emoji: other_emoji.clone(),
                emoji_codepoint: other_emoji_codepoint.clone(),
                ..Default::default()
            });

        let other_combinations = &mut metadata
            .data
            .get_mut(&other_emoji_codepoint)
            .unwrap()
            .combinations;

        if !other_combinations.contains(combination) {
            other_combinations.push(combination.clone());
        }
    }
}

pub fn get_partial_metadata() -> KitchenMetaData {
    let mut metadata: KitchenMetaData = Default::default();
    let emoji_item = get_paw_prints_combinations();
    reconstruct_metadata_from_partial_data(&mut metadata, &emoji_item);
    metadata
}
