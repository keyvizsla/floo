#!/bin/bash

# This is not to be used yet, it is a work in progress.

# -------------------------------------------
# Build FLOO and create all release files.
# Upload the contents of the dist directory.
# -------------------------------------------

APP_NAME="floo-bin"
VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d '"' -f 2)
DIST_DIR="dist"

# Only linux x86_64 supported at the moment
TARGET="x86_64-unknown-linux-musl"

rm -rf $DIST_DIR
mkdir -p $DIST_DIR

echo "📦 Building Static Release..."
cargo build --release --target "$TARGET"

BIN_PATH="target/$TARGET/release/$APP_NAME"
strip "$BIN_PATH"

# Create the archive
TARBALL="floo-linux-amd64.tar.gz"
tar -czf "$DIST_DIR/$TARBALL" -C "target/$TARGET/release" "$APP_NAME"

# Generate Checksum
echo "🔐 Generating Checksum..."
cd $DIST_DIR
sha256sum "$TARBALL" >> "checksums.txt"
cd ..
echo "✅ Done. Upload the contents of dist to the release."
