#!/usr/bin/env bash
if [ "$VCS" != "git" ]; then
    return
fi

git add .etckeeper
