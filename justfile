set positional-arguments

PYTHON_DIR := if os_family() == "windows" { "./.venv/Scripts" } else { "./.venv/bin" }
PYTHON := PYTHON_DIR + if os_family() == "windows" { "/python.exe" } else { "/python3" }
SYSTEM_PYTHON := if os_family() == "windows" { "py.exe -3" } else { "python3" }
VERSION := "0.1.0"

create-venv:
  {{SYSTEM_PYTHON}} -m venv .venv

pre-build:
  wget https://raw.githubusercontent.com/xsalazar/emoji-kitchen-backend/main/app/metadata.json -O emojikitchen.json -nc
  jq '.["data"]["1f43e"]' -c emojikitchen.json > paw_prints_kitchen_data.json

clean:
  rm emojikitchen.json
  rm paw_prints_kitchen_data.json
