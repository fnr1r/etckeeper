#!/usr/bin/env bash
vcs_init() {
    local description="$1"
    git init
    echo "$description" > .git/description
}

vcs_add_all() {
    git add --all
}

# shellcheck disable=SC2168
local _git_dir
_git_dir="$(uimport_dir)"

. "$_git_dir/write_hook.sh"

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
