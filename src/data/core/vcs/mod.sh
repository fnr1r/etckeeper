#!/usr/bin/env bash
__vcs_dir="$ETCKEEPER_DATA_DIR/core/vcs"

. "$__vcs_dir/get_dir.sh"
. "$__vcs_dir/ignore.sh"

uimport_mod "$ETCKEEPER_DATA_DIR/impls/vcs/$VCS"

# TODO: support other VCSes
if [ "$VCS" != "git" ]; then
    err "error: any VCSes other than git are unsupported for now" \
        "(selected: $VCS)"
fi

unset __vcs_dir

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
