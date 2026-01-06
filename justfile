run:
    cd chordflow_desktop && dx serve

check:
    cargo check

test:
    cargo test

build:
    cargo build -p chordflow_desktop

# Build a release macOS application bundle with icon
bundle:
    cd chordflow_desktop && dx bundle --release --platform desktop

# Build debug macOS application bundle (faster, for testing)
bundle-debug:
    cd chordflow_desktop && dx bundle --platform desktop

# Bump version (usage: just bump patch|minor|major)
bump part:
    #!/usr/bin/env bash
    set -euo pipefail

    # Get current version from chordflow_desktop/Cargo.toml
    current=$(grep '^version = ' chordflow_desktop/Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

    # Parse version parts
    IFS='.' read -r major minor patch <<< "$current"

    # Bump the appropriate part
    case "{{part}}" in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
        *)
            echo "Error: Invalid version part '{{part}}'. Use: major, minor, or patch"
            exit 1
            ;;
    esac

    new_version="${major}.${minor}.${patch}"

    echo "Bumping version: $current -> $new_version"

    # Update version in chordflow_desktop/Cargo.toml
    sed -i '' "s/^version = \"$current\"/version = \"$new_version\"/" chordflow_desktop/Cargo.toml

    # Update Cargo.lock by running cargo check
    cargo check -p chordflow_desktop

    # Add and commit
    git add chordflow_desktop/Cargo.toml Cargo.lock
    git commit -m "chore: bump version to v$new_version"

    # Create tag
    git tag -a "v$new_version" -m "Release v$new_version"

    echo "Version bumped to v$new_version"
    echo "Tag v$new_version created"
    echo ""
    echo "To push: git push && git push origin v$new_version"
