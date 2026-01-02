#!/bin/bash

set -euo pipefail

cargo build -r

PACKAGE_VERSION=$(grep '^version =' ../conda-share-cli/Cargo.toml | head -n1 | cut -d '"' -f2)
OUTPUT_NAME_CLI="conda-share_v${PACKAGE_VERSION}_aarch64-apple-darwin"
PACKAGE_VERSION=$(grep '^version =' ../conda-share-gui/Cargo.toml | head -n1 | cut -d '"' -f2)
OUTPUT_NAME_GUI="conda-share-gui_v${PACKAGE_VERSION}_aarch64-apple-darwin"

mkdir -p ../release_files

ditto -c -k --sequesterRsrc ../target/release/conda-share "../release_files/${OUTPUT_NAME_CLI}.zip"
shasum -a 256 "../release_files/${OUTPUT_NAME_CLI}.zip" > "../release_files/${OUTPUT_NAME_CLI}.sha256"

ditto -c -k --sequesterRsrc ../target/release/conda-share-gui "../release_files/${OUTPUT_NAME_GUI}.zip"
shasum -a 256 "../release_files/${OUTPUT_NAME_GUI}.zip" > "../release_files/${OUTPUT_NAME_GUI}.sha256"