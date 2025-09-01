#!/usr/bin/env bash
eecho() {
    echo "$@" 1>&2
}

# echo with NULL between args
zecho() {
    printf "%s\0" "$@"
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
