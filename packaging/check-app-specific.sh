#!/usr/bin/env bash
# App-specific checks retained alongside shared packaging lint.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

bash packaging/server/install-test.sh
# The rename rules and the pull request template name other products on purpose; a line that has
# to (a library's own identifier) says `brand-ok`.
hits=$(grep -rniE 'photoshop|psd|lienzo|pluma|illustrator|adobe|figma' packaging/ .github/ \
    --exclude=packaging-lint.yml --exclude=check-app-specific.sh --exclude=rebrand.toml \
    --exclude=pull_request_template.md | grep -v 'brand-ok' || true)
if [ -n "$hits" ]; then
  printf '%s\n' "$hits"
  echo "::error::package metadata mentions another product (see above)" >&2
  exit 1
fi

python3 - <<'PYEOF'
import yaml
manifests = []
for name in ("dev.danielmiguelt.Trama.yml", "dev.danielmiguelt.Trama.bundle.yml"):
    with open("packaging/linux/flatpak/" + name) as source:
        manifest = yaml.safe_load(source)
    assert manifest["id"] == "dev.danielmiguelt.Trama" and manifest["command"] == "trama", name
    manifests.append(manifest)
for key in ("runtime", "runtime-version", "sdk", "finish-args"):
    assert manifests[0][key] == manifests[1][key], f"Flatpak manifests disagree on {key}"
print("Trama package identity ok")
PYEOF
