#!/usr/bin/env bash

if ! command -v "$VCS" > /dev/null; then
	err "error: VCS ($VCS) is not in PATH"
fi
