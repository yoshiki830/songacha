#!/bin/sh

TO_VERSION=$1

if [ -z "$TO_VERSION" ]; then
  echo "Usage: update_version.sh VERSION"
  exit 1
fi

sed "s/^version = \".*\"/version = \"$TO_VERSION\"/" Cargo.toml > Cargo.toml.tmp
mv Cargo.toml.tmp Cargo.toml

cargo check
