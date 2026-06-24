#!/bin/sh

TAG=$1
PRODUCT_NAME=songacha
RELEASE=$PRODUCT_NAME-$TAG-x86_64-unknown-linux-gnu

cargo build --release

mkdir -p dist/$RELEASE
cp LICENSE README.md target/release/songacha dist/$RELEASE/

tar -czf dist/$RELEASE.tar.gz -C dist $RELEASE
