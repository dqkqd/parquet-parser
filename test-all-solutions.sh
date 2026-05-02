#!/usr/bin/env bash

jj log -r 'description(glob:"solution step*")' --no-graph --reversed -T 'change_id ++ "|" ++ description.first_line() ++ "\n"' \
| while IFS='|' read -r id desc; do
    echo "=== Testing: $desc ==="
    jj new "$id"
    just lint || exit 1
    cargo nextest run || exit 1
done
