#!/usr/bin/env bash
if [ -z "${managed_by_etckeeper:-}" ]; then
    _todo
fi

if ! [ -f "$IGNORE_FILE" ]; then
    return
fi

counter=0
(
    while read -r line; do
        if [ "$counter" -lt 2 ]; then
            if echo "$line" | grep -q "$managed_by_etckeeper"; then
                counter="$((counter + 1))"
            fi
            continue
        fi
        echo "$line"
    done
) <"$IGNORE_FILE" >> "$TMP_IGNORE_FILE"
