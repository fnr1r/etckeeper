#!/usr/bin/env bash
mkdir -p "$ETCKEEPER_CACHE_DIR"
"$0" list-installed > "$ETCKEEPER_CACHE_DIR/packagelist.pre-install"
