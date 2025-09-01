#!/usr/bin/env bash
vcs_get_ignore_file() {
    echo ".${VCS}ignore"
}

vcs_print_ignore_glob() {
    local glob="$1"

    case "$VCS" in
        git|hg)
            echo "${glob//#/\\#}"
            ;;
        bzr)
            echo "$glob"
            ;;
        darcs)
            _todo
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
