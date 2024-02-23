#!/usr/bin/env bash

set -euo pipefail

echo "[sudo] Ensuring dependencies are installed."

sudo snap install yq

echo "pushing tag to trigger a release"

version_tag="v$(yq e '.package.version' Cargo.toml)"

git tag -a "$version_tag" -m "$version_tag"
git push origin "$version_tag"
