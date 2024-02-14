# LGTMeow 🐾 —— 「本喵觉得很不错～」

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
```
