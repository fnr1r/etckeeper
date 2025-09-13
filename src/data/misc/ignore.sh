#!/usr/bin/env bash
# shellcheck disable=SC2034
managed_by_etckeeper="managed by etckeeper"

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
