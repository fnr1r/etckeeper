#!/usr/bin/env bash
# shellcheck disable=SC2034
message="committing changes in /etc"
if [ -z "${PM_CMDLINE}" ]; then
    message+=" after $HIGHLEVEL_PACKAGE_MANAGER run"
else
    message+=" made by $PM_CMDLINE"
fi
