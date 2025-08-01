#!/bin/bash

SRC_DIR="./challenges"
TARGET_DIR="./target"


if [ -z "$1" ]; then
    echo "Example: ./run.sh FILENAME"
    exit 1
fi

FILENAME="$1"
SRC_FILE="$SRC_DIR/$FILENAME.rs"
OUTPUT="$TARGET_DIR/$FILENAME"

if [ ! -f "$SRC_FILE" ]; then
    echo "File not found: $SRC_FILE"
    exit 1
fi

mkdir -p "$TARGET_DIR"

echo "-------------------------------------------"

echo "Compiling   =>   $SRC_FILE"

echo -e "-------------------------------------------\n"


rustc "$SRC_FILE" -o "$OUTPUT"


if [ $? -eq 0 ]; then
    "$OUTPUT"
else
    exit 1
fi
