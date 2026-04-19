#!/usr/bin/env bash
set -euo pipefail
export ROCQLIB=/mnt/c/Rocq-Platform~9.0~2025.08/lib/coq
exec /mnt/c/Rocq-Platform~9.0~2025.08/bin/rocq.exe compile "$@"
