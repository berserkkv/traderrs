#!/bin/bash
set -e

BINARY_NAME="traderrs"
TARGET_DIR="target/release"

REMOTE_USER="n"
REMOTE_HOST="193.180.208.245"
REMOTE_PATH="/home/$REMOTE_USER/$BINARY_NAME"

SERVICE_NAME="traderrs.service"
SERVICE_REMOTE_PATH="/etc/systemd/system/$SERVICE_NAME"

echo "Building release..."
cargo build --release

echo "Stripping binary..."
strip "$TARGET_DIR/$BINARY_NAME"

echo "Uploading binary..."
if ! scp "$TARGET_DIR/$BINARY_NAME" "$REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH"; then
  echo "Error uploading binary"
  exit 1
fi

echo "Uploading systemd service file..."
SERVICE_FILE_LOCAL="./$SERVICE_NAME"
if [ ! -f "$SERVICE_FILE_LOCAL" ]; then
  echo "Error: Service file $SERVICE_FILE_LOCAL does not exist locally."
  exit 1
fi

if ! scp "$SERVICE_FILE_LOCAL" "$REMOTE_USER@$REMOTE_HOST:/tmp/$SERVICE_NAME"; then
  echo "Error uploading service file"
  exit 1
fi

echo "Restarting service on remote host..."
ssh -T ${REMOTE_USER}@${REMOTE_HOST} << EOF
sudo mv /tmp/$SERVICE_NAME $SERVICE_REMOTE_PATH
sudo chmod 644 $SERVICE_REMOTE_PATH
sudo systemctl daemon-reload
sudo systemctl restart $SERVICE_NAME
sleep 2
sudo systemctl status $SERVICE_NAME --no-pager
EOF

echo "Deployment completed."
