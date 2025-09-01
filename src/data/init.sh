#!/usr/bin/env bash
. "$ETCKEEPER_DATA_DIR/utils.sh"

. "$ETCKEEPER_DATA_DIR/dirs.sh"

# shellcheck source=config.sh
. "$ETCKEEPER_DISTRO_CONFIG_FILE"

if [ -f "$ETCKEEPER_CONFIG_FILE" ]; then
    # shellcheck source=config.sh
    . "$ETCKEEPER_CONFIG_FILE"
fi

. "$ETCKEEPER_DATA_DIR/argparse.sh"

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
