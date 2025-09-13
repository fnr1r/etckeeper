#!/usr/bin/env bash
if [ -z "${managed_by_etckeeper:-}" ]; then
    _todo
fi

if ! [ -f "$IGNORE_FILE" ]; then
    return
fi

(
    while read -r line; do
        if echo "$line" | grep -q "$managed_by_etckeeper"; then
            break
        fi
        echo "$line"
    done
) <"$IGNORE_FILE" >> "$TMP_IGNORE_FILE"
