#!/usr/bin/env bash
# shellcheck disable=SC2034
pre_install="$ETCKEEPER_CACHE_DIR/packagelist.pre-install"

# TODO: Is this supposed to exit with 0 on clean?
if ! "$0" unclean; then
    _do_cleanup
    exit 0
fi
