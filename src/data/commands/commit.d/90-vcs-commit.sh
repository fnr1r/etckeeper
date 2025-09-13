#!/usr/bin/env bash
if [ "$VCS" != "git" ]; then
    _todo
fi

git commit "${POSITIONAL_ARGS[@]:1}"
