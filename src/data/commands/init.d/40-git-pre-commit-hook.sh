#!/usr/bin/env bash
if [ "$VCS" != "git" ]; then
    return
fi

hook_file=".git/hooks/pre-commit"

hook_cmd="etckeeper pre-commit"

if ! [ -x "$hook_file" ]; then
    :
elif ! grep -q "$hook_cmd" "$hook_file"; then
    eecho "etckeeper warning:" "$hook_file" \
        "needs to be manually modified to run:" \
        "$hook_cmd"
    return
fi

echo "$hook_cmd" | \
    git_write_hook pre-commit \
    "store metadata and do sanity checks"
