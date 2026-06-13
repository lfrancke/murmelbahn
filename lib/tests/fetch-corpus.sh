#!/usr/bin/env bash
#
# Fetch raw Gravitrax course bytes by code into the parser test corpus.
#
# Each code is downloaded from the murmelbahn /raw endpoint (the bytes exactly
# as the Ravensburger API returns them, only base64-decoded) and saved as
# tests/test-data/<code>.course. This is how you grow the corpus oracle: drop
# in codes from a new app version, then re-run it to see what changed.
#
#   cargo test -p murmelbahn-lib --test parse_all -- --nocapture
#
# A newly added course of a new format version shows up in the oracle either as
# Unknown(n) tags (a new piece that still parses) or as a FAILURE with an
# UNKNOWN version (the save layout of that version is not modelled).
#
# Usage:
#   ./fetch-corpus.sh CODE1 CODE2 ...
#   ./fetch-corpus.sh -f codes.txt          # one code per line, # comments ok
#   BASE_URL=http://localhost:8080 ./fetch-corpus.sh CODE   # against a local run
set -euo pipefail

BASE_URL="${BASE_URL:-https://murmelbahn.fly.dev}"
DEST="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/test-data"

codes=()
if [[ "${1:-}" == "-f" ]]; then
    [[ -n "${2:-}" ]] || { echo "error: -f needs a file argument" >&2; exit 2; }
    while read -r line; do
        line="${line%%#*}"            # strip comments
        line="$(echo "$line" | tr -d '[:space:]')"
        [[ -n "$line" ]] && codes+=("$line")
    done < "$2"
else
    codes=("$@")
fi

[[ ${#codes[@]} -gt 0 ]] || { echo "usage: $0 CODE... | -f codes.txt" >&2; exit 2; }

mkdir -p "$DEST"
ok=0 fail=0 skip=0
for code in "${codes[@]}"; do
    out="$DEST/${code}.course"
    if [[ -s "$out" ]]; then
        echo "skip   $code (already present)"
        skip=$((skip + 1))
        continue
    fi
    if curl -fsS --retry 3 "$BASE_URL/api/course/$code/raw" -o "$out"; then
        echo "ok     $code -> $(basename "$out") ($(wc -c < "$out") bytes)"
        ok=$((ok + 1))
    else
        rm -f "$out"
        echo "FAIL   $code" >&2
        fail=$((fail + 1))
    fi
done

echo "---"
echo "fetched=$ok skipped=$skip failed=$fail  (corpus: $(find "$DEST" -type f | wc -l) files)"
[[ $fail -eq 0 ]]
