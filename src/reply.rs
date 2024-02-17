use super::config::Config;
use super::kitchen::combinate::combinate_emojis;
use super::kitchen::metadata::KitchenMetaData;

#[derive(Debug, PartialEq, Clone)]
pub struct Reply {
    pub title: String,
    pub content: String,
}

fn format_lgtmeow(
    left_emoji: &str,
    right_emoji: &str,
    kitchen_link: &str,
    image_width: u32,
) -> Reply {
    Reply {
        title: format!("LGTMeow {left_emoji}+{right_emoji}"),
        content: format!("LGTMeow <img src=\"{kitchen_link}\" width=\"{image_width}\"/>"),
    }
}

pub fn load_saved_replies_from_config(config: Config, metadata: &KitchenMetaData) -> Vec<Reply> {
    config
        .emoji_codepoint_pairs
        .into_iter()
        .filter_map(|(left, right)| combinate_emojis(&left, &right, metadata))
        .map(|combination| {
            format_lgtmeow(
                &combination.left_emoji,
                &combination.right_emoji,
                &combination.g_static_url,
                config.image_size,
            )
        })
        .collect()
}
