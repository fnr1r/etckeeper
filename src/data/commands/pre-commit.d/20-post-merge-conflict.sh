#!/usr/bin/env bash
# reapply metadata after a merge conflict has been solved
if ! [ -e ".git/MERGE_HEAD" ]; then
    return
fi
"$0" metadata-apply
