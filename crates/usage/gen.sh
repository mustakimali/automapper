#!/bin/sh

set -eu pipefail

cargo check --all || true

echo "Generating rustdoc.json"
cd ../automapper-cli
cargo run -- ../usage

echo "Formatting rustdoc.json"
cd ../usage
cat rustdoc.json | jq -r '.' > rustdoc_2.json
rm rustdoc.json || true
mv rustdoc_2.json rustdoc.json

echo "Done."
