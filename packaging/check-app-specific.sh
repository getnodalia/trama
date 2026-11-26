#!/usr/bin/env bash
# App-specific checks retained alongside shared packaging lint.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

bash packaging/server/install-test.sh
# The pull request template names other products on purpose; a line that has
# to (a library's own identifier) says `brand-ok`.
hits=$(grep -rniE 'photoshop|psd|lienzo|pluma|illustrator|adobe|figma' packaging/ .github/ \
    --exclude=packaging-lint.yml --exclude=check-app-specific.sh \
    --exclude=pull_request_template.md | grep -v 'brand-ok' || true)
if [ -n "$hits" ]; then
  printf '%s\n' "$hits"
  echo "::error::package metadata mentions another product (see above)" >&2
  exit 1
fi
