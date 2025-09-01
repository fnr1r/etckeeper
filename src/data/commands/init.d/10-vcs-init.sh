#!/usr/bin/env bash

VCS_DIR="$(vcs_get_dir)"

if [ -d "$VCS_DIR" ]; then
    return
fi

VCS_DESCRIPTION="$(hostname 2>/dev/null || cat /etc/hostname) /etc repository"

vcs_init "$VCS_DESCRIPTION"
