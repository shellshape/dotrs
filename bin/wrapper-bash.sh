#!/bin/bash

if [ -n "$1" ]; then
    DOTRS_BIN="$1"
else
    DOTRS_BIN="$(which dotrs)"
fi

if [ -z "$DOTRS_BIN" ]; then
    echo "error: the dotrs executable must be installed or the path must be passed as first parameter"
    return
fi

dotrs() {
    if [ "$1" = "cd" ]; then
        cd "$("$DOTRS_BIN" $@)"
    else
        "$DOTRS_BIN" $@
    fi
}
