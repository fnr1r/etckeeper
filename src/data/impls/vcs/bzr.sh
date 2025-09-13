#!/usr/bin/env bash
vcs_init() {
    local description="$1"
    bzr init
    bzr nick "$description"
}

vcs_add_all() {
    bzr add -q .
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
