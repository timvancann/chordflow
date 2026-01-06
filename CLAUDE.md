# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ChordFlow is a GUI desktop app and TUI tool for guitarists to practice improvisation with dynamic chord progressions and a built-in metronome. Built with Rust using Dioxus for the GUI and cpal for audio processing.

## Workspace Structure

This is a Cargo workspace with two main crates:

- **chordflow_desktop**: Dioxus-based desktop GUI application with audio synthesis
- **chordflow_music_theory**: Core music theory library (notes, chords, scales, intervals, qualities)

## Development Commands

### Running the Application
```bash
# Using just (recommended)
just run

# Or directly
cd chordflow_desktop && dx serve

# For desktop platform explicitly
dx serve --platform desktop
```

### Building and Testing
```bash
# Check compilation
just check
# or
cargo check

# Run tests
just test
# or
cargo test

# Build release
cargo build --release

# Build desktop app only
just build
# or
cargo build -p chordflow_desktop
```

### Tailwind CSS (for UI styling)
```bash
# Watch mode for development
npx tailwindcss -i ./input.css -o ./chordflow_desktop/assets/tailwind.css --watch
```

## Architecture

### Audio System (chordflow_desktop/src/audio/)
- **stream.rs**: Core audio stream using cpal, manages real-time audio output, metronome clicks via rustysynth SoundFont synthesis
- Audio is initialized once at startup and leaked to maintain lifetime for the application
- Uses atomic operations and parking_lot mutexes for thread-safe state management

### State Management (chordflow_desktop/src/state/)
- **practice.rs**: Core practice state logic, manages current/next chords and practice modes
- **progression.rs**: Custom chord progression handling
- **diatonic.rs**: Diatonic chord generation
- **fourths.rs**: Circle of fourths/fifths mode
- **modes.rs**: Scale/mode selection and configuration
- **mode.rs**: Mode enum and configuration

### UI Components (chordflow_desktop/src/ui/)
- **app.rs**: Main Dioxus application component
- **top_zone/**: BPM controls, time signature, bars per cycle
- **center_stage/**: Current and next chord display
- **bottom_zone/**: Practice mode selection, progression management
- **menu_bar/**: Application menu
- **components/**: Reusable UI components
- **hooks/**: Custom Dioxus hooks

### Communication Architecture
The application uses crossbeam channels for thread-safe communication:
- `AUDIO_CMD`: UI → Audio system (start/stop, BPM changes, chord updates)
- `AUDIO_EVT`: Audio system → UI (tick events)
- `METRONOME_EVT`: Metronome → UI (bar/cycle completion)

These are defined as LazyLock statics in main.rs and accessed throughout the application.

### Music Theory Library (chordflow_music_theory/src/)
- **note.rs**: Note representation, MIDI conversion, enharmonic equivalents
- **chord.rs**: Chord construction from roots and qualities
- **interval.rs**: Musical intervals (semitone-based)
- **quality.rs**: Chord qualities (major, minor, diminished, augmented, etc.)
- **scale.rs**: Scale definitions and generation
- **accidental.rs**: Sharp/flat/natural handling

## Key Implementation Details

### SoundFont Requirement
The application requires `assets/TimGM6mb.sf2` for audio synthesis. This is loaded at startup in stream.rs:33.

### Platform-Specific Code
The main.rs uses macOS-specific window configuration (WindowBuilderExtMacOS). Desktop features are controlled via Cargo features (desktop, web, mobile).

### Audio Thread Safety
- BPM, play state, and current chord are managed via atomic types and mutexes
- Sample-accurate timing using atomic counters
- Command processing happens in a dedicated thread spawned in init_stream

## Common Development Patterns

### Adding a New Practice Mode
1. Add variant to Mode enum in state/mode.rs
2. Implement generation logic in state/practice.rs's next_chord method
3. Add UI controls in ui/bottom_zone/
4. Update mode selection in state/modes.rs if needed

### Modifying Audio Behavior
1. Add command variant to AudioCommand enum in main.rs
2. Handle command in audio/stream.rs command handler thread
3. Update UI to send new commands via AUDIO_CMD channel

### Adding UI Components
- Create component in ui/components/ or relevant zone folder
- Follow Dioxus component patterns using #[component] macro
- Use Tailwind CSS classes for styling
- Access global state via use_context or signals

## Dependencies

Key external crates:
- **dioxus**: UI framework (v0.7.2)
- **cpal**: Cross-platform audio I/O (v0.15.3)
- **rustysynth**: SoundFont synthesizer (v1.3)
- **crossbeam-channel**: Lock-free channels for audio communication (v0.5.15)
- **chordparser**: Chord parsing from string notation (v4.0.4)

## Testing

Tests are located alongside implementation files. Run with `cargo test` or `just test`.
