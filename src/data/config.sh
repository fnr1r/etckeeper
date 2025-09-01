#!/usr/bin/env bash
## etckeeper-fnrir configuration
# src/data/config.sh

## This file is sourced from dirs.sh
# shellcheck disable=SC2034

## Default settings
## These should stay the same on all distros

# Which directory should be tracked
MANAGED_DIR="/etc"
# Which VCS should be used
VCS="git"
# Options that should be passed to commit
COMMIT_OPTIONS=""
# Which remotes should be pushed on commit
PUSH_REMOTE=()

## Debug settings

# NOTE: This is just to make sure rm -rf doesn't get run during testing
# Remove this in prod.
ETCKEEPER_DIFUSE_UNINIT="yes"

## Distro-specific settings

# The high-level package manager that's being used.
# (apt, pacman, pacman-g2, yum, dnf, zypper, apk, xbps, emerge, cave, etc)
HIGHLEVEL_PACKAGE_MANAGER=apt

# The low-level package manager that's being used.
# (dpkg, rpm, pacman, pacmatic, pacman-g2, apk, xbps, cave, qlist, etc)
LOWLEVEL_PACKAGE_MANAGER=dpkg
