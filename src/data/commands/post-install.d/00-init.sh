#!/usr/bin/env bash
# shellcheck disable=SC2034
pre_install="$ETCKEEPER_CACHE_DIR/packagelist.pre-install"

_do_cleanup() {
    if [ -e "${pre_install:-}" ]; then
        rm "$pre_install"
    fi
}
