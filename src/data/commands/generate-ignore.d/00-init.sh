#!/usr/bin/env bash
comment() {
    echo "#" "$@"
}

ignore() {
    vcs_print_ignore_glob "$@"
}

newline() {
    echo
}

uimport_mod "$ETCKEEPER_DATA_DIR/misc/ignore"
