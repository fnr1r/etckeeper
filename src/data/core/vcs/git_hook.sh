#!/usr/bin/env bash

git_write_hook() {
    local hook_name="$1"
    local hook_desc="$2"

    local hook_file=".git/hooks/$hook_name"

    cat > "$hook_file" <<EOF
#!/usr/bin/env bash
# $hook_name hook for etckeeper
# $hook_desc
set -euo pipefail

################################################################################
# Do not run etckeeper inside linked worktrees. An additional worktree can be
# very useful for resolving *.rpmsave/*.rpmnew files where you are able to merge
# and check out older versions without changing the whole content of /etc.
# However while doing such work, avoid modifying .etckeeper since only the /etc
# directory should track permissions.
#
#
# $ cd /etc
# $ git worktree list
# /etc                2984704 [main]
# /root/etc.worktree  aeae148 [main.worktree]
# $ git rev-parse --git-dir
# .git
# $ cd /root/etc.worktree
# $ git rev-parse --git-dir
# /etc/.git/worktrees/etc.worktree
# $
#
################################################################################

# Using 'rev-parse' + 'grep' rather than for instance parsing output from
# 'worktree list' since the worktree command is not present in older git version
# and parsing it would be slightly more complex.
if git rev-parse --git-dir | grep -q /.git/worktrees
then
    # Inside worktree, do nothing.
    exit
fi

cd $(printf "%q" "$MANAGED_DIR")
EOF
    cat >> "$hook_file"

    chmod +x .git/hooks/pre-commit
}
