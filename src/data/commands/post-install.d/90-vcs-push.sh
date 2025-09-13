#!/usr/bin/env bash
for remote in "${PUSH_REMOTE[@]}"; do
    "$VCS" push "$remote" || true
done
