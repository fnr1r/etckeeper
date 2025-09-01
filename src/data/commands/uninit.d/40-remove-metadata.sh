#!/usr/bin/env bash
if [ "$YES_PLEASE_DELETE_MY_ENTIRE_VERSION_HISTORY" != "yes" ]; then
    exit 1
fi

uninit_remove_directory .etckeeper
