#!/bin/bash
ROOT=$(git rev-parse --show-toplevel)
README=$ROOT/README.md

echo "Updating readme at $README"
( cd $ROOT && cargo readme > $README )
