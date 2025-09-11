#!/usr/bin/env bash
pm_list_installed() {
    pacmatic -Q
}

# shellcheck disable=SC2168
local _llpm_dir
_llpm_dir="$(uimport_dir)"

. "$_llpm_dir/ignore/pacman.sh"

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
