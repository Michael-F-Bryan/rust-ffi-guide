#!/bin/sh

TARGET_DIR=./src/
CMD="mdbook build"

while inotifywait -r -e modify "$TARGET_DIR" >/dev/null 2>&1; do
    printf "$(date --iso-8601=seconds)\t$CMD\n"
    eval $CMD
done
