#!/usr/bin/env bash
if [ -z "${managed_by_etckeeper:-}" ]; then
    _todo
fi

cp "$TMP_IGNORE_FILE" "$IGNORE_FILE"

rm "$TMP_IGNORE_FILE"
