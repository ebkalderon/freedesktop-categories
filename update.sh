#!/usr/bin/env bash

#
# update.sh
#
# Generates a new version of `map.rs` and copies it to the `src` directory.
#

set -euo pipefail

cargo clean
cargo build --features generate-map

map_name="$(find ./target/ -name map.rs | head -1)"
cp "${map_name}" src/

git add -A
git commit -m 'Updated map.rs with latest spec version'
