#!/bin/bash
set -e

# ---------------------------------------- 
# Installation script to be ran by users first
# installing FLOO via a local build from source.
# Intended usage example (from root of checked out repo):
# sh ./scripts/install_local.sh
# ---------------------------------------- 

if ! command -v cargo &> /dev/null; then
    echo "❌ Error: cargo could not be found."
    echo "Rust is required to build this project from source."
    echo "Please install it via https://rustup.rs/ and try again."
    exit 1
fi

# Build and install floo-bin from source
cargo install --path .

# TODO: Install templates from local (for this we want floo-bin to output its storage path)

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
