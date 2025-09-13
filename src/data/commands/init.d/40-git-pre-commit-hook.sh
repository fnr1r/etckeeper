#!/usr/bin/env bash
if [ "$VCS" != "git" ]; then
    return
fi

hook_action="pre-commit"
hook_file=".git/hooks/${hook_action}"
hook_cmd="$(printf '%q' "$ENTRYPOINT") ${hook_action}"
hook_desc="store metadata and do sanity checks"

if [ -e "$hook_file" ] && ! grep -q "$hook_cmd" "$hook_file"; then
    eecho "etckeeper warning:" "$hook_file" \
        "needs to be manually modified to run:" \
        "$hook_cmd"
    return
fi

echo "$hook_cmd" | \
    git_write_hook "$hook_action" "$hook_desc"
