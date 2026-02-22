#!/bin/bash
set -e

# Get current version from Cargo.toml
CURRENT=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
echo "Current version: $CURRENT"
echo ""

read -p "New version (without v): " VERSION
RELEASE_TITLE=":bookmark: Release v$VERSION"
echo "Release title will be: $RELEASE_TITLE"

if [ -z "$VERSION" ] || [ -z "$RELEASE_TITLE" ]; then
  echo "❌ Version and title are required."
  exit 1
fi

VERSION="v${VERSION#v}"

echo ""
echo "  Tag:    $VERSION"
echo "  Title:  $RELEASE_TITLE"
echo "  Commit: $RELEASE_TITLE"
echo ""
read -p "Confirm? (y/n): " CONFIRM

if [ "$CONFIRM" != "y" ]; then
  echo "Cancelled."
  exit 0
fi

# Bump version in Cargo.toml and Cargo.lock
sed -i "s/^version = \".*\"/version = \"${VERSION#v}\"/" Cargo.toml
cargo generate-lockfile
git add Cargo.toml Cargo.lock
git commit -m "$RELEASE_TITLE"
git push origin HEAD

# Wait for CI to register the commit
echo ""
echo "⏳ Waiting 5s for CI to pick up the commit..."
sleep 5

# Tag and push
git tag -a "$VERSION" -m "$RELEASE_TITLE"
git push origin "$VERSION"

echo ""
echo "✅ Tag $VERSION pushed. Release workflow will start shortly."
echo "   Track it at: https://github.com/$(git remote get-url origin | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/actions"
And in auto-release.yml, simplify the post-release job by removing the Cargo.toml update: