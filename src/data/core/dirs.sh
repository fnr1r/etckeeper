#!/usr/bin/env bash
if [ -z "${XDG_DATA_HOME:-}" ]; then
    XDG_DATA_HOME="/usr/share"
fi

if [ -z "${XDG_CONFIG_HOME:-}" ]; then
    XDG_CONFIG_HOME="/etc"
fi

if [ -z "${ETCKEEPER_CONFIG_DIR:-}" ]; then
    ETCKEEPER_CONFIG_DIR="$XDG_CONFIG_HOME/etckeeper"
fi

if [ -z "${ETCKEEPER_CACHE_DIR:-}" ]; then
    ETCKEEPER_CACHE_DIR="/var/cache/etckeeper-fnrir"
fi

ETCKEEPER_DISTRO_CONFIG_FILE="$ETCKEEPER_DATA_DIR/config.sh"
ETCKEEPER_CONFIG_FILE="$ETCKEEPER_CONFIG_DIR/config.sh"

export ETCKEEPER_DISTRO_CONFIG_FILE
export ETCKEEPER_CONFIG_FILE

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
