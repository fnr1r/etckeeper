#!/usr/bin/env bash
usage() {
    echo "usage: $ME"
    echo "usage is not implemented yet"
}

RAW_CMDLINE=()
POSITIONAL_ARGS=()

argparse() {
    RAW_CMDLINE=("$@")
    while [ $# -gt 0 ]; do
    case "$1" in
        --)
            shift
            break
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        --*)
            usage 1>&2
            eecho
            eecho "Unrecognized argument $1"
            exit 1
            ;;
        -*)
            eecho "Short options (except -h) are unsupported for now"
            exit 1
            ;;
        *)
            POSITIONAL_ARGS+=("$1")
            ;;
    esac
        shift
    done
    # pass everything after -- as positional
    POSITIONAL_ARGS+=("$@")
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
