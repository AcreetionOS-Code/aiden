#!/bin/bash
# AIDEN Installation Script

set -e

echo "=== AIDEN Installation Script ==="

# Check if running as root for systemd install
if [ "$EUID" -ne 0 ] && [ "$1" = "--systemd" ]; then
    echo "Please run as root for systemd installation"
    exit 1
fi

# Build release if not exists
if [ ! -f "target/release/aiden" ]; then
    echo "Building release binary..."
    cargo build --release
fi

# Create installation directory
INSTALL_DIR="/opt/aiden"
if [ ! -d "$INSTALL_DIR" ]; then
    echo "Creating installation directory..."
    sudo mkdir -p "$INSTALL_DIR"
fi

# Copy files
echo "Installing to $INSTALL_DIR..."
sudo cp target/release/aiden "$INSTALL_DIR/"
cp -r docs "$INSTALL_DIR/"

# Set permissions
sudo chown -R root:root "$INSTALL_DIR"
sudo chmod +x "$INSTALL_DIR/aiden"

# Install systemd service
if [ "$1" = "--systemd" ]; then
    echo "Installing systemd service..."
    sudo cp aiden.service /etc/systemd/system/
    sudo systemctl daemon-reload
    sudo systemctl enable aiden
    echo "Systemd service installed and enabled!"
    echo "Run 'sudo systemctl start aiden' to start"
fi

echo "=== Installation Complete ==="
echo ""
echo "To run manually:"
echo "  cd $INSTALL_DIR && ./aiden"
echo ""
echo "To start as service:"
echo "  sudo systemctl start aiden"
echo "  sudo systemctl status aiden"
