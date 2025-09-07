#!/usr/bin/env bash
_core_dir="$ETCKEEPER_DATA_DIR/core"

. "$_core_dir/utils/mod.sh"

. "$_core_dir/dirs.sh"

# shellcheck source=../config.sh
. "$ETCKEEPER_DISTRO_CONFIG_FILE"

export -n ETCKEEPER_DISTRO_CONFIG_FILE

if [ -f "$ETCKEEPER_CONFIG_FILE" ]; then
    # shellcheck source=../config.sh
    . "$ETCKEEPER_CONFIG_FILE"
fi

export -n ETCKEEPER_CONFIG_FILE

if [ -n "${ETCKEEPER_LIBEXEC_DIR:-}" ]; then
    export PATH="$ETCKEEPER_LIBEXEC_DIR:$PATH"
    export -n ETCKEEPER_LIBEXEC_DIR
fi

. "$_core_dir/argparse.sh"

. "$_core_dir/cmdscripts.sh"

. "$_core_dir/vcs/mod.sh"

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
