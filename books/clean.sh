#!/usr/bin/env bash

use_images=$(rg -oIN 'images/[^)]+' --glob '*.md' | xargs -I{} basename {})

find src/images -type f | while read -r file; do
  filename=$(basename "$file")
  if ! echo "$use_images" | grep -qF "$filename"; then
    echo "Removing: $file"
    rm $file
  fi
done
