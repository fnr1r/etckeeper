#!/usr/bin/env bash
CMD_SCRIPTS=()
CMD_SCRIPT_NAMES=()

_cmd_script_add() {
    local file="$1"
    local filename="$2"
    CMD_SCRIPTS+=("$file")
    CMD_SCRIPT_NAMES+=("$filename")
}

cmd_script_add() {
    local file="$1"
    local filename
    if ! [ -f "$file" ]; then
        return
    fi
    filename="$(basename "$file")"
    for existing_filename in "${CMD_SCRIPT_NAMES[@]}"; do
        if [ "$filename" == "$existing_filename" ]; then
            return
        fi
    done
    _cmd_script_add "$@" "$filename"
}

cmd_script_add_dir() {
    local scripts_dir="$1"
    if ! [ -d "$scripts_dir" ]; then
        return
    fi
    for file in "$scripts_dir"/*.sh; do
        cmd_script_add "$file"
    done
}

cmd_scripts_collect() {
    local command="$1"

    local subdir="commands/$command.d"

    CMD_SCRIPTS=()
    CMD_SCRIPT_NAMES=()

    cmd_script_add_dir "$ETCKEEPER_CONFIG_DIR/$subdir"
    cmd_script_add_dir "$ETCKEEPER_DATA_DIR/$subdir"
}

cmd_scripts_sorted_add() {
    local script_name="$1"
    shift

    local filename

    for file in "$@"; do
        filename="$(basename "$file")"
        if [ "$filename" != "$script_name" ]; then
            continue
        fi
        CMD_SCRIPTS+=("$file")
    done
}

cmd_scripts_sort() {
    local cmd_script_names_unsorted=("${CMD_SCRIPT_NAMES[@]}")
    CMD_SCRIPT_NAMES=()

    mapfile -d $'\0' CMD_SCRIPT_NAMES \
        < <(zecho "${cmd_script_names_unsorted[@]}" | sort --zero-terminated)

    local cmd_scripts_unsorted=("${CMD_SCRIPTS[@]}")
    CMD_SCRIPTS=()

    for script_name in "${CMD_SCRIPT_NAMES[@]}"; do
        cmd_scripts_sorted_add "$script_name" "${cmd_scripts_unsorted[@]}"
    done
}

cmd_scripts_init() {
    local command="$1"

    cmd_scripts_collect "$command"
    cmd_scripts_sort
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
