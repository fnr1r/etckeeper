#!/usr/bin/env bash
pm_list_installed() {
    rpm -qa --qf "%|epoch?{%{epoch}}:{0}|:%{name}-%{version}-%{release}.%{arch}\n" | sort
}

pm_get_ignores() {
    comment "new and old versions of conffiles, stored by apt/rpm"
    ignore "*.rpm*"
    newline
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
