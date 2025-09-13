#!/usr/bin/env bash
if [ -z "${managed_by_etckeeper:-}" ]; then
    _todo
fi

comment "begin section $managed_by_etckeeper (do not edit this section by hand)"
newline
