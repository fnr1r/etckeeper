#!/usr/bin/env bash
set -euo pipefail

HERE="$(dirname "$(readlink -f -- "$0")")"
ME="$(basename "$0")"

if [ -z "${ETCKEEPER_DATA_DIR:-}" ]; then
    readonly ETCKEEPER_DATA_DIR="$HERE"
fi

. "$ETCKEEPER_DATA_DIR/init.sh"

main() {
    argparse "$@"

    # setup tracebacks
    trap _exit_trap EXIT
    trap _err_trap ERR

    local command="${POSITIONAL_ARGS[0]}"

    if [ "$command" == "help" ]; then
        usage
        exit 0
    fi

    err "TODO"
}

_entry() {
    set -euo pipefail
    main "$@"
    eval "exit $?"
}

_entry "$@"
