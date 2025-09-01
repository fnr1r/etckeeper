#!/usr/bin/env bash
if [ "$YES_PLEASE_DELETE_MY_ENTIRE_VERSION_HISTORY" != "yes" ]; then
    exit 1
fi

VCS_DIR="$(vcs_get_dir)"

if [ -z "$VCS_DIR" ]; then
    return
fi

uninit_remove_directory "$VCS_DIR" -rf
