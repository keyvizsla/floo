#!/bin/bash
set -e

# This is not to be used yet, it is a work in progress.

# ---------------------------------------- 
# Installation script to be ran by users first
# installing FLOO via curl.
# Intended usage example:
# curl -fsSL https://codeberg.org/KeyVizsla/floo/raw/branch/main/scripts/install_latest.sh | bash
# ---------------------------------------- 

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
  x86_64) ARCH="amd64" ;;
  *) echo "Unsupported architecture"; exit 1 ;;
esac

if ! sudo -v; then
    echo "❌ Sudo authentication failed. Exiting to prevent leaving 'dirty' files."
    exit 1
fi

echo "Starting installation for detected architecture: $OS-$ARCH..."

# Update this to the latest tag always
VERSION="v0.1.0-beta.0"
FILE="floo-$OS-$ARCH.tar.gz"
CHECKSUM_FILE="checksums.txt"
BASE_URL="https://codeberg.org/KeyVizsla/floo/releases/download/$VERSION"

echo "📥 Downloading $FILE..."
curl -LO "$BASE_URL/$FILE"
curl -LO "$BASE_URL/$CHECKSUM_FILE"

echo "🛡️ Verifying checksum..."
grep "$FILE" "$CHECKSUM_FILE" | sha256sum --check

if [ $? -eq 0 ]; then
    echo "✨ Checksum verified!"
    tar -xzf "$FILE"
    sudo mv floo-bin /usr/local/bin/
    echo "🚀 Installed successfully to /usr/local/bin/"
    floo-bin install-templates ./templates
    echo "🚀 Installed default floo templates"

    # Cleanup
    rm "$FILE" "$CHECKSUM_FILE"
    rm -rf ./templates
else
    echo "❌ CHECKSUM FAILED! The file may be corrupted or tampered with."
    exit 1
fi

# ---- Shell config setup ----

INIT_LINE='eval "$(floo-bin init)"'

detect_rc_file() {
    SHELL_NAME=$(basename "$SHELL")

    case "$SHELL_NAME" in
        zsh)
            echo "$HOME/.zshrc"
            ;;
        bash)
            if [ "$OS" = "darwin" ]; then
                echo "$HOME/.bash_profile"
            else
                echo "$HOME/.bashrc"
            fi
            ;;
        *)
            echo ""
            ;;
    esac
}

RC_FILE=$(detect_rc_file)

echo ""
echo "⚙️  Final setup step required:"
echo "Add the following line to your shell config:"
echo ""
echo "    $INIT_LINE"
echo ""

if [ -z "$RC_FILE" ]; then
    echo "⚠️  Could not detect your shell config file automatically."
    echo "Please add it manually."
    exit 0
fi

read -p "Would you like the installer to add this for you? (recommended) [Y/n]: " choice
choice=${choice:-Y}

if [[ "$choice" =~ ^[Yy]$ ]]; then
    if [ ! -f "$RC_FILE" ]; then
        touch "$RC_FILE"
    fi

    if grep -Fxq "$INIT_LINE" "$RC_FILE"; then
        echo "ℹ️  $RC_FILE already correct."
    else
        echo "" >> "$RC_FILE"
        echo "# Managed by floo installer" >> "$RC_FILE"
        echo "$INIT_LINE" >> "$RC_FILE"
        echo "✅ Added to $RC_FILE"
    fi

    echo ""
    echo "🔄 Please restart your terminal or run:"
    echo "    source $RC_FILE"
else
    echo ""
    echo "👉 Please add it manually to: $RC_FILE"
fi
