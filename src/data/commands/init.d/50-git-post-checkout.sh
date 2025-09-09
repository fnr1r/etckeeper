#!/usr/bin/env bash
if [ "$VCS" != "git" ]; then
    return
fi

hook_action="post-checkout"
hook_file=".git/hooks/${hook_action}"
hook_cmd="$(printf '%q' "$EXEC_ORG") ${hook_action}"
hook_desc="apply saved metadata"

if [ -e "$hook_file" ] && ! grep -q "$hook_cmd" "$hook_file"; then
    eecho "etckeeper warning:" "$hook_file" \
        "needs to be manually modified to run:" \
        "$hook_cmd"
    return
fi

echo "$hook_cmd" | \
    git_write_hook "$hook_action" "$hook_desc"
