use super::cli::SetupArgs;
use super::config::Config;
use super::kitchen::metadata::{Combination, KitchenMetaData};
use super::kitchen::partial_data::get_partial_metadata;
use super::kitchen::recommands::get_recommand_emoji_combinate_with_paw_prints;
use console::style;
use std::collections::HashSet;

static PAW_PRINTS_CODEPOINT: &str = "1f43e";
static PAW_PRINTS_EMOJI: &str = "🐾";

fn get_availiable_emoji_combinations(
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
        emoji_item.combinations.iter().for_each(|combination| {
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
        });
    }
    result
}

pub fn setup(args: SetupArgs) -> std::io::Result<()> {
    cliclack::intro(style(" Setup LGTMeow 🐾 ").on_cyan().black())?;

    let metadata = get_partial_metadata();
    let paw_prints_combinations =
        get_availiable_emoji_combinations(PAW_PRINTS_CODEPOINT, &metadata);
    let recommand_emoji_codepoints = get_recommand_emoji_combinate_with_paw_prints();
    let mut cliclack_items = vec![];
    let mut cliclack_initial_values = vec![];
    for (other_emoji_codepoint, other_emoji, _) in paw_prints_combinations {
        let emoji = format!("{}+{}", PAW_PRINTS_EMOJI, other_emoji);
        let is_recommand = recommand_emoji_codepoints.contains(&other_emoji_codepoint);
        let hint = if is_recommand { "recommended" } else { "" };
        if is_recommand {
            cliclack_initial_values.push(other_emoji_codepoint.clone());
        }
        cliclack_items.push((other_emoji_codepoint.clone(), emoji, hint));
    }
    let selected_emoji_codepoints;
    let image_width: u32;
    if args.default {
        image_width = 14;
        selected_emoji_codepoints = cliclack_initial_values;
    } else {
        selected_emoji_codepoints = cliclack::multiselect("Pick your favorite LGTMeow🐾")
            .initial_values(cliclack_initial_values)
            .items(&cliclack_items)
            .interact()?;

        image_width = cliclack::input("Which width do you prefer for the images? (default: 14)")
            .default_input("14")
            .interact()?;
    }

    cliclack::outro(style("Setup LGTMeow 🐾 successfully!").green())?;

    let config = Config::new(
        image_width,
        selected_emoji_codepoints
            .into_iter()
            .map(|emoji_codepoint| (PAW_PRINTS_CODEPOINT.to_string(), emoji_codepoint))
            .collect(),
    );
    config.save()?;
    Ok(())
}
