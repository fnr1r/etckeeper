#!/usr/bin/env bash
__utils_dir="$ETCKEEPER_DATA_DIR/core/utils"

. "$__utils_dir/echo.sh"
. "$__utils_dir/traceback.sh"

unset __utils_dir

err() {
    eecho "$@"
    exit 1
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
