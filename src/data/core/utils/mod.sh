#!/usr/bin/env bash
__utils_dir="$ETCKEEPER_DATA_DIR/core/utils"

. "$__utils_dir/echo.sh"
. "$__utils_dir/traceback.sh"

err() {
    eecho "$@"
    exit 1
}

. "$__utils_dir/importutils.sh"

_todo() {
    eecho "ENCOUNTERED TODO!!!"
    traceback 1
    exit 69
}

escarrprint() {
    if [ $# -lt 1 ]; then
        return
    fi
    while (($# > 1)); do
        printf '%q ' "$1"
        shift
    done
    echo "$1"
}

. "$__utils_dir/proc.sh"

does_cmd_exists() {
    local command="$1"
    command -v "$command" > /dev/null 2> /dev/null
}

create_temp() {
    local template="$1"
    if does_cmd_exists mktemp; then
        cmd=(mktemp -t "$template")
    elif does_cmd_exists tempfile; then
        cmd=(tempfile)
    else
        eecho "etckeeper: warning: can't find tempfile or mktemp"
        _todo
    fi
    "${cmd[@]}"
}

unset __utils_dir

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
