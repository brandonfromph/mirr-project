#!/usr/bin/env bash
# Wrapper: run full CI then git push.
set -euo pipefail

echo "Running ci-local.sh before git push..."
bash scripts/ci-local.sh

echo "ci-local checks passed; pushing to remote..."
git push "$@"
