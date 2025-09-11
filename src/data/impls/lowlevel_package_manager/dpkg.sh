#!/usr/bin/env bash
pm_list_installed() {
    dpkg-query -W -f '${Status}\t${Package} ${Version} ${Architecture}\n' | \
        grep -E '(ok installed|ok config-files)' | cut -f2,3
}

pm_get_ignores() {
    comment "new and old versions of conffiles, stored by dpkg"
    ignore "*.dpkg-*"
    comment "new and old versions of conffiles, stored by ucf"
    ignore "*.ucf-*"
    newline
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
