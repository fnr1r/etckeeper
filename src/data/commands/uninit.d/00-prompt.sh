#!/usr/bin/env bash
YES_PLEASE_DELETE_MY_ENTIRE_VERSION_HISTORY="n"

echo "** Warning: This will DESTROY all recorded history for:"
echo "**** '$MANAGED_DIR'"
echo "** including the $VCS repository."
echo "** ONLY TYPE IN 'yes' IF YOU KNOW WHAT YOU'RE DOING!"
echo ""

while true; do
    read -p "Are you sure you want to do this? [yes/N] " -r ans
    case "$ans" in
        yes)
            # shellcheck disable=SC2034
            YES_PLEASE_DELETE_MY_ENTIRE_VERSION_HISTORY="yes"
            break
            ;;
        [Nn]*)
            exit 1
            ;;
        *)
            continue
            ;;
    esac
done

difused() {
    if [ -z "${ETCKEEPER_DIFUSE_UNINIT:-}" ]; then
        "$@"
    else
        echo "NOT DOING:" "$@"
    fi
}

uninit_remove_directory() {
    local directory_relative_path="$1"
    local rm_args="${2:-"-r"}"

    abs_path="$MANAGED_DIR/$directory_relative_path"

    if ! [ -e "$abs_path" ]; then
        return
    fi

    if ! [ -d "$abs_path" ]; then
        eecho "$abs_path is not a directory??? NOT REMOVING"
        return
    fi

    difused rm "$rm_args" "$abs_path"
}
