#!/usr/bin/env bash
# Bump la version dans les 3 fichiers où elle apparaît.
# Usage : ./bump-version.sh 0.2.0
set -euo pipefail

if [[ -z "${1:-}" ]]; then
  echo "Usage: $0 <new-version>   (ex: $0 0.2.0)"
  exit 1
fi

NEW="$1"
cd "$(dirname "$0")"

# package.json
node -e "const f='./package.json';const j=require(f);j.version='$NEW';require('fs').writeFileSync(f,JSON.stringify(j,null,2)+'\n');"

# src-tauri/Cargo.toml
sed -i.bak -E "s/^version = \".+\"$/version = \"$NEW\"/" src-tauri/Cargo.toml && rm src-tauri/Cargo.toml.bak

# src-tauri/tauri.conf.json
node -e "const f='./src-tauri/tauri.conf.json';const j=require(f);j.version='$NEW';require('fs').writeFileSync(f,JSON.stringify(j,null,2)+'\n');"

echo "✓ Version bumpée à $NEW dans package.json, Cargo.toml, tauri.conf.json"
echo ""
echo "Prochaines étapes :"
echo "  git add -A && git commit -m \"v$NEW\""
echo "  git tag v$NEW"
echo "  git push && git push --tags"
