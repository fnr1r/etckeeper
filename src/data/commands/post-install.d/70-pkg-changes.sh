#!/usr/bin/env bash
if [ -z "${pre_install:-}" ]; then
    _todo
fi

msg_print_pkg_changes() {
    echo "Package changes:"
    "$0" list-installed | diff -U0 "$pre_install" - | tail -n+4 | grep -E '^[-+]' || true
}
