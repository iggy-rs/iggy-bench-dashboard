#!/bin/bash

set -euo

N=$1

# Fetch remote changes
git fetch

# build the server and bench
# copy bench

# Reset
git reset --hard HEAD~"$N"

# Get commit hashes in reverse order
COMMITS=$(git log --format=%H origin/master | tail -n "$N" | tac)

for COMMIT in $COMMITS; do
    # Cherry-pick commit
    git cherry-pick "$COMMIT"

    cargo build --bin iggy-server



    # Perform your operation here
    # ...
done
