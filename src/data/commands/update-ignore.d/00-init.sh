#!/usr/bin/env bash
VCS_DIR="$(vcs_get_dir)"
IGNORE_FILE="$(vcs_get_ignore_file)"

if ! [ -d "$VCS_DIR" ]; then
    err "$VCS_DIR doesn't exist! Won't create a $IGNORE_FILE file."
fi

TMP_IGNORE_FILE="$(create_temp "etckeeper-$VCS.XXXXXXXXXX")"

comment() {
    echo "#" "$@" >> "$TMP_IGNORE_FILE"
}

ignore() {
    vcs_print_ignore_glob "$@" >> "$TMP_IGNORE_FILE"
}

newline() {
    echo >> "$TMP_IGNORE_FILE"
}

cat /dev/null > "$TMP_IGNORE_FILE"

# shellcheck disable=SC2034
managed_by_etckeeper="managed by etckeeper"
