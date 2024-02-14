use super::config::Config;
use super::kitchen::combinate::combinate_emojis;
use super::kitchen::metadata::KitchenMetaData;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Clone)]
pub struct Reply {
    pub title: String,
    pub content: String,
}

impl Reply {
    fn new(title: &str, content: &str) -> Reply {
        Reply {
            title: title.to_string(),
            content: content.to_string(),
        }
    }
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

// TODO(SigureMo): Clean this unused code.
lazy_static! {
    pub static ref SAVED_REPLIES: Vec<Reply> = vec![
        Reply::new(
            "LGTMeow 🐱+📟",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20231113/u1f4df/u1f4df_u1f431.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+☁️",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20231128/u2601-ufe0f/u2601-ufe0f_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+❤️",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20230216/u1f495/u1f495_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+⭐",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20231113/u2b50/u2b50_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🌊",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20230418/u1f30a/u1f30a_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🌟",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20230127/u1f31f/u1f31f_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🌸",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20231113/u1f4ae/u1f4ae_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🎁",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20230127/u1f381/u1f381_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🎆",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20230418/u1f386/u1f386_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🎊",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20220815/u1f38a/u1f38a_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🎰",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20230126/u1f3b0/u1f3b0_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+👍",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20230803/u1f44d/u1f44d_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+💕",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20230216/u1f495/u1f495_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+💗",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20220203/u1f493/u1f493_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+💘",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20220203/u1f498/u1f498_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+💡",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20231113/u1f4a1/u1f4a1_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+💥",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20231113/u1f4a5/u1f4a5_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+📟",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20231113/u1f4df/u1f4df_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+📰",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20231113/u1f4f0/u1f4f0_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🔥",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20231113/u1f525/u1f525_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🔮",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20220506/u1f52e/u1f52e_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🗯️",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20230418/u1f5ef-ufe0f/u1f5ef-ufe0f_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🛸",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20230418/u1f6f8/u1f6f8_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🥇",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20231113/u1f947/u1f947_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🧊",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20231113/u1f9ca/u1f9ca_u1f43e.png" width="14px"/>"#
        ),
        Reply::new(
            "LGTMeow 🐾+🪤",
            r#"LGTMeow <img src="https://www.gstatic.com/android/keyboard/emojikitchen/20230418/u1faa4/u1faa4_u1f43e.png" width="14px"/>"#
        ),
    ];
}
