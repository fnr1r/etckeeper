#!/usr/bin/env bash
if [ -z "${message:-}" ]; then
    _todo
fi
if [ -z "${pre_install:-}" ]; then
    _todo
fi

msg_print_pkg_reconfig() {
    :
}

commit_verbose_do() {
    echo "$message"
    echo
    msg_print_pkg_reconfig
    # 80-pkg-changes.sh
    msg_print_pkg_changes
    # high-level pm fixes
    pm_print_pkg_changes
}

# shellcheck disable=SC2168
local commit_status=0

if [ -f "$pre_install" ]; then
    full_message="$(commit_verbose_do)"
    "$0" commit -- -F - <<< "$full_message" || commit_status=$?
else
    "$0" commit -- -m "$message" || commit_status=$?
fi

if [ "$commit_status" -ne 0 ]; then
    eecho "warning: etckeeper failed to commit changes in /etc using $VCS"
fi
