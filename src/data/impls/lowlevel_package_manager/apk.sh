#!/usr/bin/env bash
pm_list_installed() {
    apk info -v | sort
}

pm_get_ignores() {
    comment "new versions of conffiles, stored by apk"
    ignore "*.apk-new"
    newline
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
