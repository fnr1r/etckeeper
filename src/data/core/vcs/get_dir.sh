#!/usr/bin/env bash

vcs_get_dir() {
    case "$VCS" in
        git|hg|bzr)
            echo ".$VCS"
            ;;
        darcs)
            echo "_$VCS"
            ;;
        *)
            err "Unknown VCS: $VCS"
            ;;
    esac
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
