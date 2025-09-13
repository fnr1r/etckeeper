#!/usr/bin/env bash
if ! "$0" unclean; then
    return
fi
message="saving uncommitted changes in /etc prior to $HIGHLEVEL_PACKAGE_MANAGER run"
"$0" commit -- -m "$message"
