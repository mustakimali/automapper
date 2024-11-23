#!/bin/sh

set -eu pipefail

rm ../usage/rustdoc.json || true
cargo check --all || true

echo "Generating rustdoc.json"
cd ../cli
cargo run -- ../usage

echo "Formatting rustdoc.json"
cd ../usage
cat rustdoc.json | jq -r '.' > rustdoc_2.json
rm rustdoc.json || true
mv rustdoc_2.json rustdoc.json

echo "Done."
