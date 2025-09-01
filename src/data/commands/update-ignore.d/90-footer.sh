#!/usr/bin/env bash
if [ -z "${managed_by_etckeeper:-}" ]; then
    _todo
fi

comment "end section $managed_by_etckeeper"
