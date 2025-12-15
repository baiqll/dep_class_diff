#!/bin/bash
set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version is provided
if [ -z "$1" ]; then
    echo -e "${RED}Usage: $0 <version>${NC}"
    echo -e "${YELLOW}Example: $0 v0.1.0${NC}"
    exit 1
fi

VERSION=$1

# Validate version format
if [[ ! $VERSION =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Invalid version format. Use vX.Y.Z (e.g., v0.1.0)${NC}"
    exit 1
fi

echo -e "${GREEN}Preparing release $VERSION${NC}"

# Check if working directory is clean
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${RED}Working directory is not clean. Commit or stash changes first.${NC}"
    exit 1
fi

# Update version in Cargo.toml
VERSION_NUMBER=${VERSION#v}
echo -e "${YELLOW}Updating Cargo.toml version to $VERSION_NUMBER${NC}"
sed -i.bak "s/^version = \".*\"/version = \"$VERSION_NUMBER\"/" Cargo.toml
rm Cargo.toml.bak

# Update Cargo.lock
cargo build --release

# Commit version bump
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to $VERSION"

# Create tag
echo -e "${YELLOW}Creating tag $VERSION${NC}"
git tag -a "$VERSION" -m "Release $VERSION"

# Push
echo -e "${YELLOW}Pushing to remote...${NC}"
git push origin main
git push origin "$VERSION"

echo -e "${GREEN}✓ Release $VERSION created successfully!${NC}"
echo -e "${GREEN}GitHub Actions will automatically build and publish the release.${NC}"
echo -e "${YELLOW}Visit: https://github.com/baiqll/dep_class_diff/releases/tag/$VERSION${NC}"
