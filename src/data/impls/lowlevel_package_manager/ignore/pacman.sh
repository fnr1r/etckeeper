#!/usr/bin/env bash
pm_get_ignores() {
    comment "new and old versions of conffiles, stored by pacman"
    ignore "*.pacnew"
    ignore "*.pacorig"
    ignore "*.pacsave"
    newline
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
