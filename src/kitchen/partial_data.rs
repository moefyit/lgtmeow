use super::metadata::{EmojiItem, KitchenMetaData};

// Fetch all emoji-kitchen metadata in build.rs
// And prepare the partial data for the kitchen
#[cfg(feature = "emoji-paw-prints")]
static PAW_PRINTS_RAW_JSON_DATA: &str = include_str!(concat!(
    env!("OUT_DIR"),
    "/partial-kitchen-data/paw-prints.json"
));
#[cfg(feature = "emoji-cat")]
static CAT_RAW_JSON_DATA: &str =
    include_str!(concat!(env!("OUT_DIR"), "/partial-kitchen-data/cat.json"));

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
    #[cfg(feature = "emoji-paw-prints")]
    {
        let paw_prints_emoji_item: EmojiItem =
            serde_json::from_str(&PAW_PRINTS_RAW_JSON_DATA).unwrap();
        reconstruct_metadata_from_partial_data(&mut metadata, &paw_prints_emoji_item);
    }
    #[cfg(feature = "emoji-cat")]
    {
        let cat_emoji_item: EmojiItem = serde_json::from_str(&CAT_RAW_JSON_DATA).unwrap();
        reconstruct_metadata_from_partial_data(&mut metadata, &cat_emoji_item);
    }
    metadata
}
