# LGTMeow 🐾 <sub><samp>—— 「本喵觉得很不错～」</samp></sub>

Nyakku 的自用 LGTM 模板，以「LGTMeow 🐾」为基础的 Emoji Kitchen 扩充版～

## Installation

### With cargo

```bash
# If you have installed rust toolchain, you can install it via cargo
cargo install lgtmeow
# or enable `copy` feature by run
cargo install lgtmeow --features copy
```

### With pipx

Installing from PyPI requires Python 3.11 or newer. CI checks wheel installation
and CLI execution on CPython 3.11–3.15, including free-threaded 3.15t (with
prereleases allowed). These tests exercise the packaged Rust executable, not a
Python extension or its GIL behavior.

```bash
# lgtmeow has been published to pypi, you can install it via pipx
pipx install lgtmeow
# The PyPI version has `copy` feature enabled by default
# Or use uv
uv tool install lgtmeow
```

## Usage

```bash
# Setup with default preferences
lgtmeow setup --default
# Random choose a LGTMeow 🐾 from preset
lgtmeow -r
# Use it with github cli
gh pr review --approve -b "$(lgtmeow -r)"
# Copy to clipboard (need `copy` feature)
lgtmeow -r -c
```

## Acknowledgement

-  [xsalazar/emoji-kitchen](https://github.com/xsalazar/emoji-kitchen) provide a [frontend](https://emojikitchen.dev/) to view and search all available emoji-kitchen combinations. And we use it's [backend data](https://github.com/xsalazar/emoji-kitchen-backend/blob/main/app/metadata.json) to generate the preset list.
