#!/bin/bash

# Rebase all code branches from main change.
# It basically update documentation in all code branches.

set -euo pipefail
arg=${1:-}
arg=$(echo "$arg" | tr '[:upper:]' '[:lower:]')

branches=$(git branch -a | grep -P '/\d\d-' | sed 's/^.\+\///')

echo "Use $0 -f to force push the changes on all branches."
cd "$(dirname "$0")"

latest_branch=$(echo "$branches" | sort | tail -n 1)

git checkout main

if [[ "${arg[0]}" == "-f" ]]; then
  for branch in $branches; do
    echo "Pushing $branch"
    git checkout "$branch"
    git push -f
  done
else
  git checkout "$latest_branch"
  git rebase -i main --update-refs
fi

git checkout main
exit 0
