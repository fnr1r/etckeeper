#!/usr/bin/env bash
pm_list_installed() {
    pkg info -E "*"
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
