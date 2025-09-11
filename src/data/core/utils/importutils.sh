#!/usr/bin/env bash
_IMPORT_STACK=()

apush() {
    declare -n _array=$1
    shift
    _array+=("$@")
}

apop() {
    # shellcheck disable=SC2178
    declare -n _array=$1
    shift
    unset "_array[-1]"
}

uimport_dir() {
    printf %s "${_IMPORT_STACK[-1]}"
}

uimport_do() {
    local file="$1"
    shift

    local dirname
    dirname="$(dirname "$file")"
    apush _IMPORT_STACK "$dirname"
    # shellcheck disable=SC1090
    . "$file" "$@"
    apop _IMPORT_STACK
}

uimport_do_all_scripts_from_dir() {
    local dir="$1"
    local suffix="${2:-".sh"}"
    shift 2

    for file in "$dir"/*"$suffix"; do
        uimport_do "$file" "$@"
    done
}

uimport_mod() {
    local path="$1"
    shift

    for file in "$path.sh" "$path/mod.sh"; do
        if ! [ -e "$file" ]; then
            continue
        fi
        uimport_do "$file" "$@"
        return
    done

    if [ -d "$path" ]; then
        uimport_do_all_scripts_from_dir "$path" "$@"
        return
    fi

    err "module $path doesn't exist"
    return 1
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
