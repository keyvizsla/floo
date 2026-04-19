#!/bin/bash

export FLOO_OUTPUT_FILE=$(mktemp)
echo $FLOO_OUTPUT_FILE
cargo run
[ -s "$FLOO_OUTPUT_FILE" ] && eval "$(cat $FLOO_OUTPUT_FILE)"
rm -f "$FLOO_OUTPUT_FILE"
unset FLOO_OUTPUT_FILE
