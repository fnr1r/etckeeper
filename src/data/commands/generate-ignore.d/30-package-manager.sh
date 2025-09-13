#!/usr/bin/env bash
if [ "$LOWLEVEL_PACKAGE_MANAGER" = "dpkg" ]; then
    comment "new and old versions of conffiles, stored by dpkg"
    ignore "*.dpkg-*"
    comment "new and old versions of conffiles, stored by ucf"
    ignore "*.ucf-*"
    newline
elif [ "$LOWLEVEL_PACKAGE_MANAGER" = "rpm" ]; then
    comment "new and old versions of conffiles, stored by apt/rpm"
    ignore "*.rpm*"
    newline
elif [ "$LOWLEVEL_PACKAGE_MANAGER" = "pacman-g2" ] || [ "$LOWLEVEL_PACKAGE_MANAGER" = "pacman" ] || [ "$LOWLEVEL_PACKAGE_MANAGER" = "pacmatic" ]; then
    comment "new and old versions of conffiles, stored by pacman"
    ignore "*.pacnew"
    ignore "*.pacorig"
    ignore "*.pacsave"
    newline
elif [ "$LOWLEVEL_PACKAGE_MANAGER" = "apk" ]; then
    comment "new versions of conffiles, stored by apk"
    ignore "*.apk-new"
    newline
elif [ "$LOWLEVEL_PACKAGE_MANAGER" = "xbps" ]; then
    comment "new versions of conffiles, stored by xbps"
    ignore "*.new-*_[0-9]*"
    newline
elif [ "$LOWLEVEL_PACKAGE_MANAGER" = "qlist" ] || [ "$LOWLEVEL_PACKAGE_MANAGER" = "cave" ]; then
    comment "new and old versions of conffiles, stored by emerge"
    ignore "._cfg*"
    newline
fi
