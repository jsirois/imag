#!/usr/bin/env bash

# This SHOULD NOT be executed before the CI step.

# Make sure we are on the PR-branch
git checkout $TRAVIS_COMMIT^2

HDR="$(cat .lgpl_header)"
missing=0
for file in $(git ls-files | grep "\.rs$")
do
    if [[ "$(head -n 18 "$file")" == "$HDR" ]]
    then
        echo "LGPL Header present in $file"
    else
        echo "LGPL Header MISSING in $file"
        missing=1
    fi
done

if [[ $missing -eq 1 ]]; then
    echo "LGPL Header missing in at least one file."
    exit 1
fi

