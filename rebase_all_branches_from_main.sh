#!/bin/bash

# Rebase all code branches from main change.
# It basically update documentation in all code branches.

set -euo pipefail
arg=${1:-}
arg=$(echo "$arg" | tr '[:upper:]' '[:lower:]')

branches=$(git branch | grep -P '^\s+\d\d-')

echo "Use $0 -f to rebase and force push the changes"
git checkout main

for branch in $branches; do
  echo "Rebasing $branch"
  git checkout "$branch"
  git rebase main

  if [[ "${arg[0]}" == "-f" ]]; then
    git push -f
  fi
done

git checkout main
exit 0
