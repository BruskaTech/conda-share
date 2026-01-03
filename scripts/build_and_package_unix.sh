#!/usr/bin/env bash
set -euo pipefail

: "${TARGET:?TARGET is not set}"

TAG="${GITHUB_REF_NAME:-local}"
DIST_DIR="dist"
META_FILE="$(mktemp)"

cleanup() { rm -f "$META_FILE"; }
trap cleanup EXIT

# Get metadata once (includes the REAL target directory)
cargo metadata --no-deps --format-version=1 > "$META_FILE"

TARGET_DIR="$(jq -r '.target_directory' "$META_FILE")"
if [[ -z "$TARGET_DIR" || "$TARGET_DIR" == "null" ]]; then
  echo "ERROR: Could not determine target_directory from cargo metadata."
  exit 1
fi

RELEASE_DIR="${TARGET_DIR}/${TARGET}/release"

echo "Target directory: ${TARGET_DIR}"
echo "Release directory: ${RELEASE_DIR}"

# List all workspace (package, bin) pairs
# lines: "<package>\t<bin>"
BIN_LIST="$(jq -r '
  .packages[]
  | .name as $pkg
  | .targets[]
  | select(.kind | index("bin"))
  | "\($pkg)\t\(.name)"
' "$META_FILE")"

if [[ -z "${BIN_LIST}" ]]; then
  echo "ERROR: No workspace binaries found (no targets with kind=bin)."
  exit 1
fi

# Build ONCE for the whole workspace
echo ""
echo "== Building workspace once: --release --target ${TARGET} =="
cargo build --release --workspace --target "${TARGET}"

# Package all bins from the release dir
mkdir -p "${DIST_DIR}"

while IFS=$'\t' read -r pkg bin; do
  [[ -z "${bin}" ]] && continue

  BIN_PATH="${RELEASE_DIR}/${bin}"
  if [[ ! -f "${BIN_PATH}" ]]; then
    echo "ERROR: Expected binary not found: ${BIN_PATH}"
    echo "Hint: check whether this target actually produces this bin for ${TARGET}."
    exit 1
  fi

  ZIP_NAME="${bin}-${TAG}-${TARGET}.zip"

  echo ""
  echo "== Packaging: package=${pkg} bin=${bin} =="
  rm -rf "${DIST_DIR:?}/${bin}"
  mkdir -p "${DIST_DIR}/${bin}"

  cp "${BIN_PATH}" "${DIST_DIR}/${bin}/${bin}"
  chmod +x "${DIST_DIR}/${bin}/${bin}"

  ( cd "${DIST_DIR}/${bin}" && zip -r "../../${ZIP_NAME}" "${bin}" )

  echo "Created ${ZIP_NAME}"
done < <(printf '%s\n' "${BIN_LIST}")

echo ""
echo "Done. Zips in repo root:"
ls -1 *.zip
