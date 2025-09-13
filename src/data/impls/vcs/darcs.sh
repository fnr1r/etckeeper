#!/usr/bin/env bash
vcs_init() {
    local description="$1"
    darcs initialize
    echo "$description" > _darcs/prefs/motd
}

vcs_add_all() {
    local ret_code=0
    local out
    out="$(darcs add -qr . 2>&1)" || ret_code=$?
    if [ "$ret_code" -eq "0" ]; then
        return
    fi
    if [ "$ret_code" -eq "2" ] && [ "${out%No files were added}" != "$out" ]; then
        return;
    fi
    eecho "etckeeper warning: darcs add failed"
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
