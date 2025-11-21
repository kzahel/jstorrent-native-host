#!/bin/bash
# Install script for JSTorrent Native Host

INSTALL_DIR="$HOME/.local/lib/jstorrent-native"
MANIFEST_DIR="$HOME/.config/google-chrome/NativeMessagingHosts"
MANIFEST_TEMPLATE="manifests/com.jstorrent.native.json.template"
BINARY_NAME="jstorrent-native-host"

# Create install directory
mkdir -p "$INSTALL_DIR"

# Copy binary
cp "$BINARY_NAME" "$INSTALL_DIR/"
chmod 755 "$INSTALL_DIR/$BINARY_NAME"

# Copy uninstall script
cp "installers/linux/uninstall.sh" "$INSTALL_DIR/"
chmod 755 "$INSTALL_DIR/uninstall.sh"

# Create manifest directory
mkdir -p "$MANIFEST_DIR"

# Generate manifest
MANIFEST_DEST="$MANIFEST_DIR/com.jstorrent.native.json"
BINARY_PATH="$INSTALL_DIR/$BINARY_NAME"

sed "s|HOST_PATH_PLACEHOLDER|$BINARY_PATH|g" "$MANIFEST_TEMPLATE" > "$MANIFEST_DEST"
chmod 644 "$MANIFEST_DEST"

echo "JSTorrent Native Host installed successfully."
echo "To uninstall, run: $INSTALL_DIR/uninstall.sh"
