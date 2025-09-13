#!/usr/bin/env bash
get_parent_of_pid() {
    local pid="$1"
    cat "/proc/$pid/stat" | cut -d ' ' -f 4
}

get_cmdline_of_pid() {
    local pid="$1"
    declare -n _cmdline="$2"
    while IFS= read -r -d $'\0' arg; do
        _cmdline+=("$arg")
    done < "/proc/$pid/cmdline"
}

print_cmdline_of_pid() {
    local pid="$1"
    local cmdline=()
    get_cmdline_of_pid "$pid" cmdline
    escarrprint "${cmdline[@]}"
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
