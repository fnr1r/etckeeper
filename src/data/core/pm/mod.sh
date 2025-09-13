#!/usr/bin/env bash
__pm_dir="$ETCKEEPER_DATA_DIR/core/pm"

pm_print_pkg_changes() {
    :
}

uimport_mod "$ETCKEEPER_DATA_DIR/impls/highlevel_package_manager/$HIGHLEVEL_PACKAGE_MANAGER"
uimport_mod "$ETCKEEPER_DATA_DIR/impls/lowlevel_package_manager/$LOWLEVEL_PACKAGE_MANAGER"

unset __pm_dir

get_pm_uid() {
    local pm_child_uid=""
    case "$HIGHLEVEL_PACKAGE_MANAGER" in
        pacman|zypper)
            pm_child_uid="$PPID"
            ;;
        apt|dnf|yum)
            pm_child_uid="$(get_parent_of_pid "$PPID")"
            ;;
        *)
            ;;
    esac
    if [ -z "$pm_child_uid" ]; then
        return
    fi
    get_parent_of_pid "$pm_child_uid"
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
