#!/usr/bin/env bash
set -euo pipefail

: "${TARGET:?TARGET is not set}"

TAG="${GITHUB_REF_NAME:-local}"
DIST_DIR="dist"

# Get all (package, bin) pairs in the workspace
# Output lines like: "<package>\t<bin>"
mapfile -t PAIRS < <(
  cargo metadata --no-deps --format-version=1 \
  | jq -r '.packages[] | .name as $pkg | .targets[]
          | select(.kind | index("bin"))
          | "\($pkg)\t\(.name)"'
)

if [[ ${#PAIRS[@]} -eq 0 ]]; then
  echo "No workspace binaries found (no targets with kind=bin)."
  exit 1
fi

echo "Found ${#PAIRS[@]} binaries in workspace."

mkdir -p "${DIST_DIR}"

for pair in "${PAIRS[@]}"; do
  pkg="${pair%%$'\t'*}"
  bin="${pair#*$'\t'}"

  echo ""
  echo "== Building: package=${pkg} bin=${bin} target=${TARGET} =="

  cargo build --release --target "${TARGET}" -p "${pkg}" --bin "${bin}"

  BIN_PATH="target/${TARGET}/release/${bin}"
  if [[ ! -f "${BIN_PATH}" ]]; then
    echo "ERROR: Expected binary not found at ${BIN_PATH}"
    exit 1
  fi

  ZIP_NAME="${bin}-${TAG}-${TARGET}.zip"

  rm -rf "${DIST_DIR:?}/${bin}"
  mkdir -p "${DIST_DIR}/${bin}"
  cp "${BIN_PATH}" "${DIST_DIR}/${bin}/${bin}"
  chmod +x "${DIST_DIR}/${bin}/${bin}"

  ( cd "${DIST_DIR}/${bin}" && zip -r "../../${ZIP_NAME}" "${bin}" )

  echo "Created ${ZIP_NAME}"
done

echo ""
echo "Done. Zips in repo root:"
ls -1 *.zip
