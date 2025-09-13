#!/usr/bin/env bash
PM_PID="$(get_pm_uid)"

if [ -z "$PM_PID" ]; then
    PM_CMDLINE=""
    return
fi

# shellcheck disable=SC2034
PM_CMDLINE="$(print_cmdline_of_pid "$PM_PID")"
