#!/usr/bin/env bash
pm_print_pkg_changes() {
    # packages remain in qlist during postrm
    if [ "${EBUILD_PHASE:-}" != "postrm" ]; then
        return
    fi
    echo "-$CATEGORY/$PF"
}

if ! (return 2> /dev/null); then
    echo "ERROR: $0 is not a runnable bash script!" 1>&2
    exit 1
fi
