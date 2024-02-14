# LGTMeow 🐾 <sub><samp>—— 「本喵觉得很不错～」</samp></sub>

Nyakku 的自用 LGTM 模板，以「LGTMeow 🐾」为基础的 Emoji Kitchen 扩充版～

## Installation

```bash
cargo install lgtmeow
```

## Usage

```bash
# Setup with default preferences
lgtmeow setup --default
# Random choose a LGTMeow 🐾 from preset
lgtmeow -r
# Use it with github cli
gh pr review --approve -b "$(lgtmeow -r)"
# Copy to clipboard (need `copy` feature, run `cargo install lgtmeow --features copy` to enable it)
lgtmeow -r -c
```

## Acknowledgement

-  [xsalazar/emoji-kitchen](https://github.com/xsalazar/emoji-kitchen) provide a [frontend](https://emojikitchen.dev/) to view and search all available emoji-kitchen combinations. And we use it's [backend data](https://github.com/xsalazar/emoji-kitchen-backend/blob/main/app/metadata.json) to generate the preset list.
