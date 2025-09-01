#!/usr/bin/env bash
if [ "$VCS" != "hg" ]; then
    return
fi

comment "use glob syntax"
echo "syntax: glob" >> "$TMP_IGNORE_FILE"
newline
