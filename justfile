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

# Bump the workspace version, commit and tag (usage: just release patch|minor|major)
release part:
    cargo release {{part}} --execute
