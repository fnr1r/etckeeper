#!/usr/bin/env bash
__pm_dir="$ETCKEEPER_DATA_DIR/core/pm"

uimport_mod "$ETCKEEPER_DATA_DIR/impls/highlevel_package_manager/$HIGHLEVEL_PACKAGE_MANAGER"
uimport_mod "$ETCKEEPER_DATA_DIR/impls/lowlevel_package_manager/$LOWLEVEL_PACKAGE_MANAGER"

unset __pm_dir

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
