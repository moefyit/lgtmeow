use crate::cli::args::SetupArgs;
use crate::config::Config;
use crate::kitchen::metadata::{Combination, KitchenMetaData};
use crate::kitchen::partial_data::get_partial_metadata;
#[cfg(feature = "emoji-cat")]
use crate::kitchen::recommends::RECOMMEND_EMOJI_CODEPOINTS_COMBINE_WITH_CAT;
#[cfg(feature = "emoji-paw-prints")]
use crate::kitchen::recommends::RECOMMEND_EMOJI_CODEPOINTS_COMBINE_WITH_PAW_PRINTS;
use console::style;
use std::collections::HashSet;

#[cfg(feature = "emoji-paw-prints")]
static PAW_PRINTS_CODEPOINT: &str = "1f43e";
#[cfg(feature = "emoji-paw-prints")]
static PAW_PRINTS_EMOJI: &str = "🐾";

#[cfg(feature = "emoji-cat")]
static CAT_CODEPOINT: &str = "1f431";
#[cfg(feature = "emoji-cat")]
static CAT_EMOJI: &str = "🐱";

fn get_available_emoji_combinations(
    emoji_codepoint: &str,
    metadata: &KitchenMetaData,
) -> Vec<(String, String, Combination)> {
    let mut result = vec![];
    let mut added_emoji_codepoints = HashSet::new();
    if !metadata
        .known_supported_emoji
        .contains(&emoji_codepoint.to_string())
    {
        return result;
    }
    if let Some(emoji_item) = metadata.data.get(emoji_codepoint) {
        emoji_item
            .combinations
            .iter()
            .for_each(|(_, combinations)| {
                for combination in combinations {
                    let (other_emoji_codepoint, other_emoji) =
                        if combination.left_emoji_codepoint == emoji_codepoint {
                            (
                                combination.right_emoji_codepoint.to_string(),
                                combination.right_emoji.to_string(),
                            )
                        } else {
                            (
                                combination.left_emoji_codepoint.to_string(),
                                combination.left_emoji.to_string(),
                            )
                        };
                    if added_emoji_codepoints.contains(&other_emoji_codepoint) {
                        return;
                    }
                    added_emoji_codepoints.insert(other_emoji_codepoint.clone());
                    result.push((other_emoji_codepoint, other_emoji, combination.clone()));
                }
            });
    }
    result
}

fn get_cliclack_inquire_config<'a>(
    emoji_codepoint: &str,
    emoji_emoji: &str,
    recommend_combine_emoji_codepoints: &'static [&str],
    metadata: &'a KitchenMetaData,
) -> (Vec<String>, Vec<(String, String, &'a str)>) {
    let mut cliclack_items = vec![];
    let mut cliclack_initial_values = vec![];
    let emoji_combinations = get_available_emoji_combinations(emoji_codepoint, metadata);
    for (other_emoji_codepoint, other_emoji, _) in emoji_combinations {
        let emoji = format!("{emoji_emoji}+{other_emoji}");
        let is_recommend =
            recommend_combine_emoji_codepoints.contains(&other_emoji_codepoint.as_str());
        let hint = if is_recommend { "recommended" } else { "" };
        if is_recommend {
            cliclack_initial_values.push(other_emoji_codepoint.clone());
        }
        cliclack_items.push((other_emoji_codepoint.clone(), emoji, hint));
    }
    (cliclack_initial_values, cliclack_items)
}

pub fn setup(args: SetupArgs) -> std::io::Result<()> {
    cliclack::intro(style(" Setup LGTMeow 🐾 ").on_cyan().black())?;

    let metadata = get_partial_metadata();

    #[cfg(feature = "emoji-paw-prints")]
    let (paw_prints_cliclack_initial_values, paw_prints_cliclack_items) =
        get_cliclack_inquire_config(
            PAW_PRINTS_CODEPOINT,
            PAW_PRINTS_EMOJI,
            RECOMMEND_EMOJI_CODEPOINTS_COMBINE_WITH_PAW_PRINTS,
            &metadata,
        );
    #[cfg(feature = "emoji-cat")]
    let (cat_cliclack_initial_values, cat_cliclack_items) = get_cliclack_inquire_config(
        CAT_CODEPOINT,
        CAT_EMOJI,
        RECOMMEND_EMOJI_CODEPOINTS_COMBINE_WITH_CAT,
        &metadata,
    );

    #[cfg(feature = "emoji-paw-prints")]
    let selected_emoji_codepoints_combine_with_paw_prints;
    #[cfg(feature = "emoji-cat")]
    let selected_emoji_codepoints_combine_with_cat;

    let image_width: u32;
    if args.default {
        image_width = 14;
        #[cfg(feature = "emoji-paw-prints")]
        {
            selected_emoji_codepoints_combine_with_paw_prints = paw_prints_cliclack_initial_values;
        }
        #[cfg(feature = "emoji-cat")]
        {
            selected_emoji_codepoints_combine_with_cat = cat_cliclack_initial_values;
        }
    } else {
        #[cfg(feature = "emoji-paw-prints")]
        {
            selected_emoji_codepoints_combine_with_paw_prints =
                cliclack::multiselect("Pick your favorite LGTMeow🐾")
                    .initial_values(paw_prints_cliclack_initial_values)
                    .items(&paw_prints_cliclack_items)
                    .interact()?;
        }
        #[cfg(feature = "emoji-cat")]
        {
            selected_emoji_codepoints_combine_with_cat =
                cliclack::multiselect("Pick your favorite LGTMeow🐱")
                    .initial_values(cat_cliclack_initial_values)
                    .items(&cat_cliclack_items)
                    .interact()?;
        }

        image_width = cliclack::input("Which width do you prefer for the images? (default: 14)")
            .default_input("14")
            .interact()?;
    }

    cliclack::outro(style("Setup LGTMeow 🐾 successfully!").green())?;

    let mut emoji_codepoint_pairs = vec![];
    #[cfg(feature = "emoji-paw-prints")]
    emoji_codepoint_pairs.extend(
        selected_emoji_codepoints_combine_with_paw_prints
            .into_iter()
            .map(|emoji_codepoint| (PAW_PRINTS_CODEPOINT.to_string(), emoji_codepoint)),
    );
    #[cfg(feature = "emoji-cat")]
    emoji_codepoint_pairs.extend(
        selected_emoji_codepoints_combine_with_cat
            .into_iter()
            .map(|emoji_codepoint| (CAT_CODEPOINT.to_string(), emoji_codepoint)),
    );
    let config = Config::new(image_width, emoji_codepoint_pairs);
    config.save()?;
    Ok(())
}
