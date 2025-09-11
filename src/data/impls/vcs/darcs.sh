#!/usr/bin/env bash
vcs_init() {
    local description="$1"
    darcs initialize
    echo "$description" > _darcs/prefs/motd
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
