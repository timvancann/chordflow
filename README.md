# 🎸 ChordFlow

![Logo](icons/web/icon-192.png)

ChordFlow is a desktop app designed to help guitarists/musicians
practice improvisation and master the guitar neck by providing dynamic chord progressions with a built-in metronome.  

Grab the latest [release](https://github.com/timvancann/chordflow/releases)

![ChordFlow](docs/screenshot.png)

## ✨ Features

- 🎵 Metronome with Custom Sounds – Supports SoundFont-based metronome ticks.
- 🔄 Random Chord Generation – Generate new chords every bar to improve improvisation skills.
- 📊 Visual Progress Bar – Displays the current beat and bar progress.
- 🎼 Real-Time Chord Display – Shows the current and upcoming chord.
- ⚙️ Customizability – Users can supply their own SoundFont for metronome ticks and chord sounds.
- 🎥 [Desktop GUI demo](https://www.youtube.com/watch?v=X5V7tlbOBbY)

## 📦 Installation

1. Build from Source

```bash
git clone https://github.com/timvancann/chordflow
cd chordflow
cargo build --release
```

2. Grab the latest [release](https://github.com/timvancann/chordflow/releases)

### Opening the app on macOS

Releases are ad-hoc signed and not notarized by Apple, so the first launch is blocked by
Gatekeeper with a message like "ChordFlow is damaged and can't be opened". The app is fine,
macOS just refuses unnotarized downloads. After installing, remove the quarantine flag once:

```bash
xattr -d com.apple.quarantine /Applications/ChordFlow.app
```

After that the app opens normally.

## 🚀 Usage

Install [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/)

```dash
cd chordflow_desktop
dx serve
```

## 🏗️ Roadmap

- [ ] Fix Linux release
- [ ] Add more scales (e.g. melodic minor)
- [x] Better feedback and UI on custom progressions
- [ ] Allow dynamically update the number of beats per bar
- [x] Use [Dioxux](https://dioxuslabs.com/) to create a GUI native app

## 🤝 Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

1. Fork the repo
2. Create a new branch (git checkout -b feature-name)
3. Commit changes (git commit -m "Added cool feature")
4. Push to branch (git push origin feature-name)
5. Open a pull request
