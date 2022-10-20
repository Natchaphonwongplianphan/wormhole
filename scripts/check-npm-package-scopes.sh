#!/usr/bin/env bash

# This script checks to ensure that all our NPM packages have an appropriate scope.
#
find . -type f -name 'package.json' -print0 | xargs -n1 -r -0 /bin/bash -c 'printf "[$@]"; jq  ".name" "$@";' '' | egrep -v "^\[.*\]null$" | egrep -v '^\[.*\]"(@certusone/|@wormhole-foundation/)'
if [ $? -eq 0 ]; then
   echo "[!] Unscoped npm packages" >&2
   exit 1
else
   echo "[+] No unscoped npm packages"
fi
