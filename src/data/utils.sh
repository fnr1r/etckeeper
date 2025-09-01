#!/usr/bin/env bash
. "$ETCKEEPER_DATA_DIR/utils/traceback.sh"

eecho() {
    echo "$@" 1>&2
}

err() {
    eecho "$@"
    exit 1
}

# echo with NULL between args
zecho() {
    printf "%s\0" "$@"
}

_err_traceback() {
    eecho "Traceback:"
    eecho "  Depth: ${BASH_SOURCE[*]}"
    for file in "${BASH_SOURCE[@]}"; do
        eecho "  $file"
    done
    for aaa in "${BASH_LINENO[@]}"; do
        eecho "$aaa"
    done
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
