#!/bin/bash

set -e

cargo build --release

mkdir -p "$HOME/.local/bin"

echo "Moving binary to $HOME/.local/bin/floo-bin"
cp target/release/floo-bin "$HOME/.local/bin/floo-bin"

BASHRC="$HOME/.bashrc"
MARKER="# [FLOO-FUNCTION-START]"

if grep -qF "$MARKER" "$BASHRC"; then
    echo "Floo function already exists in $BASHRC. Skipping append."
else
    echo "Adding floo function to $BASHRC..."
    cat << EOF >> "$BASHRC"

$MARKER
floo() {
    local tmp_file
    tmp_file=\$(mktemp)
    export FLOO_OUTPUT_FILE="\$tmp_file"
    
    # Run the binary (ensure ~/.local/bin is in your PATH)
    command floo-bin "\$@"
    
    if [ -s "\$tmp_file" ]; then
        . "\$tmp_file"
    fi
    rm -f "\$tmp_file"
    unset FLOO_OUTPUT_FILE
}
# [FLOO-FUNCTION-END]
EOF
fi

echo "----------------------------------------------------"
echo "Installation successful!"
echo "Please run: source ~/.bashrc"
echo "Then you can use the command: floo"
