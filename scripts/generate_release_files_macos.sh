#!/bin/bash

set -euo pipefail

cargo build -r

PACKAGE_VERSION=$(grep '^version =' ../Cargo.toml | head -n1 | cut -d '"' -f2)
PACKAGE_VERSION="v${PACKAGE_VERSION}"
OUTPUT_NAME="conda-share_${PACKAGE_VERSION}_aarch64-apple-darwin"

mkdir -p ../release_files

ditto -c -k --sequesterRsrc ../target/release/conda-share "../release_files/${OUTPUT_NAME}.zip"
shasum -a 256 "../release_files/${OUTPUT_NAME}.zip" > "../release_files/${OUTPUT_NAME}.sha256"