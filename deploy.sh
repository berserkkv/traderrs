#!/bin/bash
set -e

BINARY_NAME="traderrust"
TARGET_DIR="target/release"

REMOTE_USER="n"
REMOTE_HOST=""
REMOTE_PATH="/home/$REMOTE_USER/$BINARY_NAME"


echo "1. Building release..."
cargo build --release

echo "2. Stripping binary..."
strip "$TARGET_DIR/$BINARY_NAME"

echo "3. Uploading to server..."
scp "$TARGET_DIR/$BINARY_NAME" "$REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH"

echo "4. Running on server..."
ssh "$REMOTE_USER@$REMOTE_HOST" "chmod +x $REMOTE_PATH && $REMOTE_PATH"


