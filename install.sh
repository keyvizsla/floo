#!/usr/bin/env bash

set -euo pipefail

BIN_NAME="floo-bin"
INSTALL_DIR="$HOME/.local/bin"
BIN_PATH="$INSTALL_DIR/$BIN_NAME"

FUNC_MARKER="# [FLOO-FUNCTION-START]"
PATH_MARKER="# [FLOO-PATH-START]"

detect_rc_file() {
    local shell_name
    shell_name="$(basename "${SHELL:-}")"

    case "$shell_name" in
        zsh)
            echo "$HOME/.zshrc"
            ;;
        bash)
            # On some macOS bash setups, .bash_profile is used instead of .bashrc.
            if [ -f "$HOME/.bashrc" ] || [ ! -f "$HOME/.bash_profile" ]; then
                echo "$HOME/.bashrc"
            else
                echo "$HOME/.bash_profile"
            fi
            ;;
        *)
            # Fallback for uncommon shells
            echo "$HOME/.profile"
            ;;
    esac
}

echo "Building release binary..."
cargo build --release

mkdir -p "$INSTALL_DIR"

echo "Installing binary to $BIN_PATH"
cp "target/release/$BIN_NAME" "$BIN_PATH"
chmod +x "$BIN_PATH"

RC_FILE="$(detect_rc_file)"
touch "$RC_FILE"

# Add ~/.local/bin to PATH (idempotent)
if grep -qF "$PATH_MARKER" "$RC_FILE" 2>/dev/null; then
    echo "PATH block already exists in $RC_FILE. Skipping."
else
    echo "Adding PATH block to $RC_FILE..."
    cat << 'EOF' >> "$RC_FILE"

# [FLOO-PATH-START]
export PATH="$HOME/.local/bin:$PATH"
# [FLOO-PATH-END]
EOF
fi

# Add floo wrapper function (idempotent)
if grep -qF "$FUNC_MARKER" "$RC_FILE" 2>/dev/null; then
    echo "floo function already exists in $RC_FILE. Skipping."
else
    echo "Adding floo function to $RC_FILE..."
    cat << 'EOF' >> "$RC_FILE"

# [FLOO-FUNCTION-START]
floo() {
    local tmp_file
    tmp_file="$(mktemp)"
    export FLOO_OUTPUT_FILE="$tmp_file"

    if [ ! -x "$HOME/.local/bin/floo-bin" ]; then
        echo "Error: $HOME/.local/bin/floo-bin not found or not executable."
        rm -f "$tmp_file"
        unset FLOO_OUTPUT_FILE
        return 1
    fi

    command "$HOME/.local/bin/floo-bin" "$@"

    if [ -s "$tmp_file" ]; then
        . "$tmp_file"
    fi

    rm -f "$tmp_file"
    unset FLOO_OUTPUT_FILE
}
# [FLOO-FUNCTION-END]
EOF
fi

echo "----------------------------------------------------"
echo "Installation successful!"
echo "Detected shell config: $RC_FILE"
echo "Please run:"
echo "  source \"$RC_FILE\""
echo "Then you can use:"
echo "  floo"