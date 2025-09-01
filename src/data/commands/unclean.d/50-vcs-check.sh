#!/usr/bin/env bash
if [ "$VCS" = git ]; then
    [ -n "$(git status --porcelain)" ]
elif [ "$VCS" = hg ]; then
    ! hg status 2>&1 | wc -l | grep -q "^0$"
elif [ "$VCS" = bzr ]; then
    ! bzr version-info --custom --template="{clean}\n" | grep -q "^1$"
elif [ "$VCS" = darcs ]; then
    darcs whatsnew -l >/dev/null
fi
