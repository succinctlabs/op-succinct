#!/usr/bin/env bash

set -euo pipefail

programs=(range range-altda range-eigenda)
patches=(
  "sha2 RustCrypto-hashes"
  "sha3 RustCrypto-hashes"
  "crypto-bigint RustCrypto-bigint"
  "k256 elliptic-curves"
  "p256 elliptic-curves"
  "substrate-bn bn"
)

for program in "${programs[@]}"; do
  dependency_tree=$(cargo tree --locked -p "$program" --all-features --edges normal --prefix none --format '{p}')

  for patch in "${patches[@]}"; do
    read -r package repository <<< "$patch"
    expected_source="https://github.com/sp1-patches/${repository}?"
    found=false

    while IFS= read -r dependency; do
      [[ "$dependency" == "$package v"* ]] || continue
      found=true

      if [[ "$dependency" != *"$expected_source"* ]]; then
        echo "error: $program resolves an unpatched $package dependency:" >&2
        echo "       $dependency" >&2
        echo "       expected every $package dependency from $expected_source" >&2
        exit 1
      fi
    done <<< "$dependency_tree"

    if [[ "$found" != true ]]; then
      echo "error: $program does not resolve $package" >&2
      exit 1
    fi
  done
done

echo "All range programs resolve SP1-patched crypto dependencies."
