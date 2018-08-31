#!/usr/bin/env bash

set -euo pipefail

cargo clean
cargo build --features generate-map

map_name="$(find ./target/ -name map.rs | head -1)"
cp "${map_name}" src/

git commit -m 'Updated map.rs with latest spec version'
