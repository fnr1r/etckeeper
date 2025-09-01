#!/usr/bin/env bash

vcs_init() {
    local description="$1"
    case "$VCS" in
        git)
            git init
            echo "$description" > .git/description
            ;;
        hg)
            hg init
            echo  "[web]" > .hg/hgrc
            echo  "description = $description" >> .hg/hgrc
            ;;
        bzr)
            bzr init
            bzr nick "$description"
            ;;
        darcs)
            darcs initialize
            echo "$description" > _darcs/prefs/motd
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
