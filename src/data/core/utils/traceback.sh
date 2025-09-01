#!/usr/bin/env bash
#
# Tracebacks in bash
# https://docwhat.org/tracebacks-in-bash/
#
# Just take the code between the "cut here" lines
# and put it in your own program.
#
# Written by Christian Höltje
# Donated to the public domain in 2013
#
# Slightly reformatted by me

_showed_traceback=f

function _exit_trap() {
    local _ec="$?"
    if [[ $_ec != 0 && ${_showed_traceback} != t ]]; then
        traceback 1
    fi
}

function _err_trap() {
    local _ec="$?"
    local _cmd="${BASH_COMMAND:-unknown}"
    traceback 1
    _showed_traceback=t
    echo "The command ${_cmd} exited with exit code ${_ec}." 1>&2
}

function traceback() {
    # Hide the traceback() call.
    local -i start=$((${1:-0} + 1))
    local -i end=${#BASH_SOURCE[@]}
    local -i i=0
    local -i j=0

    echo "Traceback (last called is first):" 1>&2
    for ((i = start; i < end; i++)); do
        j=$((i - 1))
        local function="${FUNCNAME[$i]}"
        local file="${BASH_SOURCE[$i]}"
        local line="${BASH_LINENO[$j]}"
        echo "     ${function}() in ${file}:${line}" 1>&2
    done
}
